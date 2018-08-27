use prelude::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use glutin::{KeyboardInput, ElementState, Event, WindowEvent};
use np::world::World;
use np::object::RigidBody;

use game::{KeyboardState, Entity, EntityBuilder};
use render::{Camera, Render, Light};
use settings::Settings;

pub const FONT_SIZE: f32 = 20.0;

pub type EntityId = u32;

/// Gravity type of the simulation
#[derive(Copy, Clone)]
pub enum Gravity {
	/// Each object attracts each other object, scaled by a specified amount.
	Relative(f32),
	/// Each object is attracted in a constant direction
	Constant(Vector3<f32>),
	/// No gravity is applied
	None,
}

pub trait TickCallback {
	fn tick(&mut self, state: &mut GameState, dt: f32, settings: &Settings, events: &[Event], mouse_moved: Vector2<f64>);
}
impl<F> TickCallback for F where F: FnMut(&mut GameState, f32, &Settings, &[Event], Vector2<f64>) {
	fn tick(&mut self, state: &mut GameState, dt: f32, settings: &Settings, events: &[Event], mouse_moved: Vector2<f64>) {
		self(state, dt, settings, events, mouse_moved)
	}
}
pub trait RenderCallback {
	fn render(&mut self, state: &mut GameState, r: &mut Render, fps: u32);
}
impl<F> RenderCallback for F where F: FnMut(&mut GameState, &mut Render, u32) {
	fn render(&mut self, state: &mut GameState, r: &mut Render, fps: u32) {
		self(state, r, fps)
	}
}

/// Holds the state of the game
pub struct GameState {
	world: World<f32>,
	gravity: Gravity,
	next_free_id: EntityId,
	entities: HashMap<EntityId, Entity>,
	keyboard_state: KeyboardState,
	camera: Camera,
	light: Light,
	ambient_light: Vector4<f32>,
	wireframe_mode: bool,
	tick_callback: Option<Rc<RefCell<TickCallback>>>,
	render_callback: Option<Rc<RefCell<RenderCallback>>>,
}
impl GameState {
	/// Constructs a new GameState with the specified initial camera position, and gravity state.
	/// 
	/// The main light in the scene is initialized to off. Use `set_light` to specify the light.
	pub fn new(cam: Camera, g: Gravity) -> GameState {
		GameState {
			world: World::new(),
			gravity: g,
			next_free_id: 0,
			entities: HashMap::new(),
			keyboard_state: KeyboardState::new(),
			camera: cam,
			light: Light::off(),
			ambient_light: Vector4::new(0.05, 0.05, 0.05, 1.0),
			wireframe_mode: false,
			tick_callback  : None,
			render_callback: None,
		}
	}
	
	pub fn set_ambient_light(&mut self, ambient_light: Vector4<f32>) {
		self.ambient_light = ambient_light;
	}
	
	pub fn light(&self) -> &Light {
		&self.light
	}
	
	pub fn set_light(&mut self, l: Light) {
		self.light = l;
	}
	
	pub fn camera(&self) -> &Camera {
		&self.camera
	}
	
	/// Sets the tick callback. This will be called every physics tick.
	pub fn set_tick_callback(&mut self, callback: Option<Rc<RefCell<TickCallback>>>) {
		self.tick_callback = callback;
	}
	
	/// Sets the tick callback. This will be called every frame render.
	pub fn set_render_callback(&mut self, callback: Option<Rc<RefCell<RenderCallback>>>) {
		self.render_callback = callback;
	}
	
	/// Adds an entity to the world
	pub fn add_entity(&mut self, build: EntityBuilder) -> EntityId {
		let id = self.next_free_id;
		self.next_free_id += 1;
		
		let e = build.build_world(&mut self.world);
		self.entities.insert(id, e);
		id
	}
	
	/// Gets a reference to the entity with the specified id
	pub fn get_entity(&self, id: EntityId) -> Option<&Entity> {
		self.entities.get(&id)
	}
	
	/// Gets a mutable reference to the entity with the specified id
	pub fn get_entity_mut(&mut self, id: EntityId) -> Option<&mut Entity> {
		self.entities.get_mut(&id)
	}
	
	/// Gets a reference to the entity's body with the specified id
	pub fn get_entity_rigid_body(&self, id: EntityId) -> Option<&RigidBody<f32>> {
		self.entities.get(&id).and_then(|e| self.world.rigid_body(e.body()))
	}
	
	/// Gets a mutable reference to the entity's body with the specified id
	pub fn get_entity_rigid_body_mut<'a>(&'a mut self, id: EntityId) -> Option<&'a mut RigidBody<f32>> {
		let body = self.entities.get(&id).map(|e| e.body());
		if let Some(body) = body {
			self.world.rigid_body_mut(body)
		} else {
			None
		}
	}
	
	/// Remove an entity from the simulation.
	/// If an entity with the ID specified existed, returns that entity.
	pub fn remove_entity(&mut self, id: &EntityId) -> Option<Entity> {
		if let Some(e) = self.entities.remove(id) {
			e.remove_world(&mut self.world);
			Some(e)
		} else {
			None
		}
	}
	
	/// Processes a tick of the game state.
	/// 
	/// - `dt` is the number of seconds to process.
	/// - `settings` are the current game settings.
	/// - `events` is a list of events that occured since last frame.
	/// - `mouse_moved` is how much the mouse has moved (in screen pixels) since the last update.
	pub fn tick(&mut self, dt: f32, settings: &Settings, events: &mut Vec<Event>, mouse_moved: Vector2<f64>) {
		// Call callback
		{
			let call = self.tick_callback.clone();
			if let Some(call) = call {
				let mut call = call.borrow_mut();
				call.tick(self, dt, settings, &*events, mouse_moved);
			}
		}
		
		// m/s
		let speed = 4.0 * dt;
		
		for e in events.drain(..) {
			match e {
				Event::WindowEvent{event: WindowEvent::KeyboardInput{input: KeyboardInput{state:key_state, virtual_keycode: Some(code), ..}, ..}, ..} => {
					self.keyboard_state.process_event(key_state, code);
					if key_state == ElementState::Pressed {
						if Some(code) == settings.wireframe_toggle {
							self.wireframe_mode = !self.wireframe_mode;
							if self.wireframe_mode {
								info!("Wireframe mode enabled");
							} else {
								info!("Wireframe mode disabled");
							}
						}
					}
				},
				_ => {}
			}
		}
		
		// Translate camera based on keyboard state
		let mut trans = Vector3::new(0.0, 0.0, 0.0);
		if self.keyboard_state.is_pressed(&settings.forward) {
			trans = trans + Vector3::new(0.0, 0.0, -speed);
		}
		if self.keyboard_state.is_pressed(&settings.backward) {
			trans = trans + Vector3::new(0.0, 0.0,  speed);
		}
		if self.keyboard_state.is_pressed(&settings.left) {
			trans = trans + Vector3::new(-speed, 0.0, 0.0);
		}
		if self.keyboard_state.is_pressed(&settings.right) {
			trans = trans + Vector3::new( speed, 0.0, 0.0);
		}
		if self.keyboard_state.is_pressed(&settings.up) {
			trans = trans + Vector3::new(0.0,  speed, 0.0);
		}
		if self.keyboard_state.is_pressed(&settings.down) {
			trans = trans + Vector3::new(0.0, -speed, 0.0);
		}
		self.camera.translate(trans);
		self.camera.mouse_moved(mouse_moved);
		
		if !settings.paused {
			// info!("=== Entities ===");
			// for (i, e) in self.entities.iter() {
			// 	if let Some(body) = self.world.rigid_body(e.body()) {
			// 		let pos = body.position().translation.vector;
			// 		let vel = body.velocity().linear;
			// 		let mass = body.augmented_mass().mass();
			// 		info!("{}: mass: {:.2}, pos:[{:.2}, {:.2}, {:.2}], vel:[{:.2}, {:.2}, {:.2}]", i, mass, pos.x, pos.y, pos.z, vel.x, vel.y, vel.z);
			// 	}
			// }
			
			// Apply gravity to all non-static entities.
			match self.gravity {
				Gravity::Relative(g) => self.calculate_gravity(g),
				Gravity::Constant(v) => self.world.set_gravity(v),
				Gravity::None        => self.world.set_gravity(Vector3::new(0.0, 0.0, 0.0)),
			}
			
			// Tick world
			self.world.set_timestep(dt);
			self.world.step();
		}
	}
	
	/// Calculates relative gravity for all the entities in the scene.
	fn calculate_gravity(&mut self, g: f32) {
		// info!("Calculating gravity");
		let id_vec: Vec<_> = self.entities.keys().cloned().collect();
		let mut ids = id_vec.iter();
		loop {
			let a_id = match ids.next() {
				Some(a) => a,
				None => break,
			};
			for b_id in ids.clone() {
				let f = {
					let a = self.world.rigid_body(self.entities[&a_id].body());
					let b = self.world.rigid_body(self.entities[&b_id].body());
					
					if let (Some(a), Some(b)) = (a, b) {
						if a.is_static() && b.is_static() {
							continue;
						}
						let a_mass = a.augmented_mass().mass();
						let b_mass = b.augmented_mass().mass();
						
						// Get unit vector from a to b 
						let mut v = b.position().translation.vector - a.position().translation.vector;
						let len_sq = v.norm_squared();
						v = v / len_sq.sqrt();
						
						// Calc force.
						let lin_force = v * ((g * a_mass * b_mass) / len_sq);
						// info!("Calculate gravity {} <-> {}: {:6.2?}", a_id, b_id, lin_force);
						Force3::linear(lin_force)
					} else {
						continue;
					}
				};
				// Apply force
				self.world.rigid_body_mut(self.entities[&a_id].body()).unwrap().apply_force(&f);
				let f = Force3::linear(-f.linear);
				self.world.rigid_body_mut(self.entities[&b_id].body()).unwrap().apply_force(&f);
			}
		}
	}
	
	/// Renders the GameState using the specified render handler.
	/// 
	/// `fps` is the current frames per second.
	pub fn render(&mut self, r: &mut Render, fps: u32) {
		r.set_camera(self.camera);
		r.set_ambient_light(self.ambient_light);
		r.set_light(self.light);
		r.set_wireframe_mode(self.wireframe_mode);
		
		for e in self.entities.values() {
			e.render(r, &self.world);
		}
		
		r.draw_str(&format!("{} FPS", fps), 10.0, 10.0, FONT_SIZE);
		
		// Call callback
		{
			let call = self.render_callback.clone();
			if let Some(call) = call {
				let mut call = call.borrow_mut();
				call.render(self, r, fps);
			}
		}
		
		r.swap();
	}
}
