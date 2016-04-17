use prelude::*;
use std::collections::HashMap;

use glutin::{ElementState, VirtualKeyCode, Event};
use np::world::World;

use game::{KeyboardState, Entity, EntityBuilder};
use render::{Camera, Render, Light};
use settings::Settings;

const FONT_SIZE: f32 = 24.0;

pub type EntityId = u32;

/// Gravity type of the simulation
#[derive(Copy, Clone)]
pub enum Gravity {
	/// Each object attracts each other object, scaled by a specified amount.
	Relative(f32),
	/// Each object is attracted in a constant direction
	Constant(Vec3<f32>),
	/// No gravity is applied
	None,
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
	ambient_light: Vec4<f32>,
	wireframe_mode: bool,
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
			ambient_light: Vec4::new(0.05, 0.05, 0.05, 1.0),
			wireframe_mode: false,
		}
	}
	
	pub fn set_ambient_light(&mut self, ambient_light: Vec4<f32>) {
		self.ambient_light = ambient_light;
	}
	
	pub fn set_light(&mut self, l: Light) {
		self.light = l;
	}
	
	pub fn camera(&self) -> &Camera {
		&self.camera
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
	pub fn get_entity<'a>(&'a self, id: &EntityId) -> Option<&'a Entity> {
		self.entities.get(id)
	}
	
	/// Gets a mutable reference to the entity with the specified id
	pub fn get_entity_mut<'a>(&'a mut self, id: &EntityId) -> Option<&'a mut Entity> {
		self.entities.get_mut(id)
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
	pub fn tick<I: Iterator<Item=Event>>(&mut self, dt: f32, settings: &Settings, events: I, mouse_moved: Vec2<i32>) {
		// m/s
		let speed = 4.0 * dt;
		
		for e in events {
			match e {
				Event::KeyboardInput(ElementState::Pressed, _, Some(code)) => {
					if Some(code) == settings.wireframe_toggle {
						self.wireframe_mode = !self.wireframe_mode;
						if self.wireframe_mode {
							info!("Wireframe mode enabled");
						} else {
							info!("Wireframe mode disabled");
						}
					}
				},
				_ => {}
			}
		}
		
		// Translate camera based on keyboard state
		let mut trans = Vec3::new(0.0, 0.0, 0.0);
		if self.keyboard_state.is_pressed(&settings.forward) {
			trans = trans + Vec3::new(0.0, 0.0, -speed);
		}
		if self.keyboard_state.is_pressed(&settings.backward) {
			trans = trans + Vec3::new(0.0, 0.0,  speed);
		}
		if self.keyboard_state.is_pressed(&settings.left) {
			trans = trans + Vec3::new(-speed, 0.0, 0.0);
		}
		if self.keyboard_state.is_pressed(&settings.right) {
			trans = trans + Vec3::new( speed, 0.0, 0.0);
		}
		if self.keyboard_state.is_pressed(&settings.up) {
			trans = trans + Vec3::new(0.0,  speed, 0.0);
		}
		if self.keyboard_state.is_pressed(&settings.down) {
			trans = trans + Vec3::new(0.0, -speed, 0.0);
		}
		self.camera.translate(trans);
		self.camera.mouse_moved(mouse_moved);
		
		if !settings.paused {
			/*info!("=== Entities ===");
			for (i, e) in self.entities.iter() {
				let body = e.components()[0].body().borrow();
				let pos = body.position().translation;
				let vel = body.lin_vel();
				info!("{}: pos:[{}, {}, {}], vel:[{}, {}, {}]", i, pos.x, pos.y, pos.z, vel.x, vel.y, vel.z);
			}*/
			
			// Apply gravity to all non-static entities.
			match self.gravity {
				Gravity::Relative(g) => self.calculate_gravity(g),
				Gravity::Constant(v) => self.world.set_gravity(v),
				Gravity::None        => self.world.set_gravity(Vec3::new(0.0, 0.0, 0.0)),
			}
			
			// Tick world
			self.world.step(dt);
		}
	}
	
	/// Calculates relative gravity for all the entities in the scene.
	pub fn calculate_gravity(&mut self, g: f32) {
		let id_vec: Vec<_> = self.entities.keys().cloned().collect();
		let mut ids = id_vec.iter();
		loop {
			let a_id = match ids.next() {
				Some(a) => a,
				None => break,
			};
			for b_id in ids.clone() {
				let f = {
					let a = self.get_entity(&a_id).map(|b| b.body().borrow()).unwrap();
					let b = self.get_entity(&b_id).map(|b| b.body().borrow()).unwrap();
					
					if !a.can_move() && !b.can_move() {
						continue;
					}
					let (a_mass, b_mass) = {
						if a.inv_mass() == 0.0 || b.inv_mass() == 0.0 {
							continue;
						}
						(1.0 / a.inv_mass(), 1.0 / b.inv_mass())
					};
					
					// Get unit vector from a to b 
					let mut v = b.position().translation - a.position().translation;
					let len_sq = v.sqnorm();
					v = v / len_sq.sqrt();
					
					// Calc force.
					v * ((g * a_mass * b_mass) / len_sq)
				};
				// Apply force
				self.entities.get_mut(&a_id).map(|e| e.body().borrow_mut().apply_central_impulse(f));
				self.entities.get_mut(&b_id).map(|e| e.body().borrow_mut().apply_central_impulse(-f));
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
			e.render(r);
		}
		
		r.draw_str(&format!("{} FPS", fps), 10.0, 10.0, FONT_SIZE);
		
		r.swap();
	}
}
