use prelude::*;
use std::collections::HashMap;

use glutin::{ElementState, VirtualKeyCode};
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
	next_free_id: EntityId,
	entities: HashMap<EntityId, Entity>,
	camera: Camera,
	wireframe_mode: bool,
	gravity: Gravity,
	world: World<f32>,
	light: Light,
}
impl GameState {
	/// Constructs a new GameState with the specified initial camera position, and gravity state.
	/// 
	/// The light is set to off. Use `set_light` to specify the light.
	pub fn new(cam: Camera, g: Gravity) -> GameState {
		GameState {
			next_free_id: 0,
			entities: HashMap::new(),
			camera: cam,
			wireframe_mode: false,
			gravity: g,
			world: World::new(),
			light: Light::off(),
		}
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
	/// - `keys` is the list of keys pressed/released since the last update.
	/// - `keyboard_state` is the current keyboard state.
	/// - `mouse_state` is how much the mouse has moved (in screen pixels) since the last update.
	pub fn tick(&mut self, dt: f32, settings: &Settings, keys: &[(ElementState, VirtualKeyCode)], keyboard_state: &KeyboardState, mouse_state: (i32, i32)) {
		// m/s
		let speed = 4.0 * dt;
		
		// Translate camera based on keyboard state
		let mut trans = Vec3::new(0.0, 0.0, 0.0);
		if keyboard_state.is_pressed(&settings.forward) {
			trans = trans + Vec3::new(0.0, 0.0, -speed);
		}
		if keyboard_state.is_pressed(&settings.backward) {
			trans = trans + Vec3::new(0.0, 0.0,  speed);
		}
		if keyboard_state.is_pressed(&settings.left) {
			trans = trans + Vec3::new(-speed, 0.0, 0.0);
		}
		if keyboard_state.is_pressed(&settings.right) {
			trans = trans + Vec3::new( speed, 0.0, 0.0);
		}
		if keyboard_state.is_pressed(&settings.up) {
			trans = trans + Vec3::new(0.0,  speed, 0.0);
		}
		if keyboard_state.is_pressed(&settings.down) {
			trans = trans + Vec3::new(0.0, -speed, 0.0);
		}
		self.camera.translate(trans);
		self.camera.mouse_moved(mouse_state.0, mouse_state.1);
		for &(s, ref key) in keys.iter() {
			if s == ElementState::Pressed {
				if Some(*key) == settings.wireframe_toggle {
					self.wireframe_mode = !self.wireframe_mode;
					if self.wireframe_mode {
						info!("Wireframe mode enabled");
					} else {
						info!("Wireframe mode disabled");
					}
				}
			}
		}
		
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
		r.set_light(self.light);
		r.set_wireframe_mode(self.wireframe_mode);
		
		for e in self.entities.values() {
			e.render(r);
		}
		
		r.draw_str(&format!("{} FPS", fps), 10.0, 10.0, FONT_SIZE);
		
		r.swap();
	}
}
