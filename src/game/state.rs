use std::rc::Rc;
use std::collections::HashMap;
use std::collections::hash_map::Keys;
use std::iter;

use glium::backend::Context;
use na::{Norm, Vec3};
use nc::shape::Ball;
use np::world::World;
use np::object::{RigidBody, RigidBodyHandle};

use game::{KeyboardState, Entity, GameState};
use render::{Camera, Render, SimpleMesh, ColoredMesh, Color};
use settings::Settings;

const FONT_SIZE: f32 = 24.0;

/// Gravity type of the simulation
#[derive(Copy, Clone)]
pub enum Gravity {
	/// Each object attracts each other object
	Relative,
	/// Each object is attracted in a constant direction
	Constant(Vec3<f32>),
}

pub struct State {
	next_free_id: u64,
	bodies: HashMap<u64, RigidBodyHandle<f32>>,
	camera: Camera,
	gravity: Gravity,
	world: World<f32>,
}
impl State {
	pub fn new(cam: Camera, g: Gravity) -> State {
		State {
			next_free_id: 0,
			bodies: HashMap::new(),
			camera: cam,
			gravity: g,
			world: World::new(),
		}
	}
	
	pub fn gen_balls(ctx: &Rc<Context>, cam: Camera) -> State {
		// TODO: RigidBody builder to sort out all of this mess
		let sphere = Rc::new(SimpleMesh::sphere(ctx, 4));
		
		let red   = box ColoredMesh::new(sphere.clone(), Color::RED);
		let green = box ColoredMesh::new(sphere.clone(), Color::GREEN);
		let blue  = box ColoredMesh::new(sphere.clone(), Color::BLUE);
		
		let mut state = GameState::new(cam, Gravity::Relative);
		let r = Entity::new(red  , 1.0);
		let r = state.add_entity(RigidBody::new_dynamic(Ball::new(1.0), 1.0, 1.0, 0.0), box r);
		state.get_body(&r).unwrap().borrow_mut().set_translation(Vec3::new(5.0, 0.0,  0.0));
		state.get_body(&r).unwrap().borrow_mut().set_lin_vel(Vec3::new(0.0, 1.0, -1.0));
		let g = Entity::new(green, 1.0);
		let g = state.add_entity(RigidBody::new_dynamic(Ball::new(1.0), 1.0, 1.0, 0.0), box g);
		state.get_body(&g).unwrap().borrow_mut().set_translation(Vec3::new(0.0, 0.0, -5.0));
		state.get_body(&g).unwrap().borrow_mut().set_lin_vel(Vec3::new(1.0, -1.0, 1.0));
		let b = Entity::new(blue , 1.0);
		let b = state.add_entity(RigidBody::new_dynamic(Ball::new(1.0), 1.0, 1.0, 0.0), box b);
		state.get_body(&b).unwrap().borrow_mut().set_translation(Vec3::new(0.0, 5.0,  0.0));
		state.get_body(&b).unwrap().borrow_mut().set_lin_vel(Vec3::new(-1.0, 1.0, 1.0));
		state
	}
	
	pub fn gen_solar(ctx: &Rc<Context>, cam: Camera) -> State {
		let sphere = Rc::new(SimpleMesh::sphere(ctx, 4));
		
		let yellow = box ColoredMesh::with_scale(sphere.clone(), Color::YELLOW, 1.0);
		let green  = box ColoredMesh::with_scale(sphere.clone(), Color::GREEN , 0.3684);
		let red    = box ColoredMesh::with_scale(sphere.clone(), Color::RED   , 0.07937);
		
		let mut state = GameState::new(cam, Gravity::Relative);
		let sun  = Entity::new(yellow, 100.0);
		let sun = state.add_entity(RigidBody::new_dynamic(Ball::new(1.0    ), 1.0, 1.0, 0.0), box sun);
		state.get_body(&sun).unwrap().borrow_mut().set_translation(Vec3::new(0.0, 0.0, 0.0));
		state.get_body(&sun).unwrap().borrow_mut().set_lin_vel(Vec3::new(0.0, 0.0, 1.7505));
		let earth = Entity::new(green, 5.0);
		let earth = state.add_entity(RigidBody::new_dynamic(Ball::new(0.3684 ), 1.0, 1.0, 0.0), box earth);
		state.get_body(&earth).unwrap().borrow_mut().set_translation(Vec3::new(10.0, 0.0, 0.0));
		state.get_body(&earth).unwrap().borrow_mut().set_lin_vel(Vec3::new(0.0, 0.0, -35.0));
		let mercury = Entity::new(red, 0.05);
		let mercury = state.add_entity(RigidBody::new_dynamic(Ball::new(0.07937), 1.0, 1.0, 0.0), box mercury);
		state.get_body(&mercury).unwrap().borrow_mut().set_translation(Vec3::new(4.0, 0.0, 0.0));
		state.get_body(&mercury).unwrap().borrow_mut().set_lin_vel(Vec3::new(0.0, 0.0, -15.0));
		
		state
	}
	
	pub fn camera(&self) -> &Camera {
		&self.camera
	}
	
	pub fn entities(&self) -> ::std::collections::hash_map::Values<u64, RigidBodyHandle<f32>> {
		self.bodies.values()
	}
	
	/// Adds an entity to the world
	pub fn add_entity(&mut self, mut body: RigidBody<f32>, e: Box<Entity>) -> u64 {
		let id = self.next_free_id;
		*body.user_data_mut() = Some(e);
		let h = self.world.add_body(body);
		self.bodies.insert(id, h);
		self.next_free_id += 1;
		id
	}
	
	/// Gets the rigid body with the specified id
	pub fn get_body<'a>(&'a self, id: &u64) -> Option<&'a RigidBodyHandle<f32>> {
		self.get_item(id).map(|p| p.0)
	}
	
	/// Gets the entity with the specified id
	pub fn get_entity<'a>(&'a self, id: &u64) -> Option<&'a Entity> {
		self.get_item(id).map(|p| p.1)
	}
	
	/// Gets the pair (RigidBody, Entity) with the specified id
	pub fn get_item<'a>(&'a self, id: &u64) -> Option<(&'a RigidBodyHandle<f32>, &'a Entity)> {
		unsafe {
			match self.bodies.get(id) {
				Some(ref b) => {
					let b_ptr = b.as_unsafe_cell().get();
					match (*b_ptr).user_data() {
						&Some(ref any) => match any.downcast_ref::<Entity>() {
							Some(e) => Some((b, e)),
							None => None,
						},
						&None => None,
					}
				},
				None => None,
			}
		}
	}
	
	pub fn ids(&self) -> iter::Cloned<Keys<u64, RigidBodyHandle<f32>>> {
		self.bodies.keys().cloned()
	}
	
	/// Removed the entity with the specified ID from the simulation.
	/// If an entity with that ID existed, returns the entity removed.
	pub fn remove_body(&mut self, id: u64) -> Option<RigidBodyHandle<f32>> {
		if let Some(b) = self.bodies.remove(&id) {
			self.world.remove_body(&b);
			Some(b)
		} else {
			None
		}
	}
	
	pub fn tick(&mut self, dt: f32, settings: &Settings, keyboard: &KeyboardState, mouse_state: (i32, i32)) {
		// m/s
		let speed = 4.0 * dt;
		
		// Translate camera based on keyboard state
		let mut trans = Vec3::new(0.0, 0.0, 0.0);
		if keyboard.is_pressed(&settings.forward) {
			trans = trans + Vec3::new(0.0, 0.0, -speed);
		}
		if keyboard.is_pressed(&settings.backward) {
			trans = trans + Vec3::new(0.0, 0.0,  speed);
		}
		if keyboard.is_pressed(&settings.left) {
			trans = trans + Vec3::new(-speed, 0.0, 0.0);
		}
		if keyboard.is_pressed(&settings.right) {
			trans = trans + Vec3::new( speed, 0.0, 0.0);
		}
		if keyboard.is_pressed(&settings.up) {
			trans = trans + Vec3::new(0.0,  speed, 0.0);
		}
		if keyboard.is_pressed(&settings.down) {
			trans = trans + Vec3::new(0.0, -speed, 0.0);
		}
		self.camera.translate(trans);
		self.camera.mouse_moved(mouse_state.0, mouse_state.1);
		
		if !settings.paused {
			// Apply gravity to all non-static entities.
			match self.gravity {
				Gravity::Relative => {
					let mut ids = self.bodies.keys().cloned();
					loop {
						let a_id = match ids.next() {
							Some(a) => a,
							None => break,
						};
						for b_id in ids.clone() {
							const G: f32 = 1.0;
							
							let (mut a_body, a_ent) = self.get_item(&a_id).map(|(b, e)| (b.borrow_mut(), e)).unwrap();
							let (mut b_body, b_ent) = self.get_item(&b_id).map(|(b, e)| (b.borrow_mut(), e)).unwrap();
							
							if !a_body.can_move() && !b_body.can_move() {
								continue;
							}
							
							// Get unit vector from a to b 
							let mut v = b_body.position().translation - a_body.position().translation;
							let len_sq = v.sqnorm();
							v = v / len_sq.sqrt();
							
							// Calc && apply the force.
							let f = v * ((G * a_ent.mass() * b_ent.mass()) / len_sq);
							a_body.apply_central_impulse(f);
							b_body.apply_central_impulse(-f);
						}
					}
				},
				Gravity::Constant(v) => self.world.set_gravity(v),
			}
			
			// Tick world
			self.world.step(dt);
		}
	}

	pub fn render(&mut self, r: &mut Render, fps: u32) {
		r.set_camera(self.camera);
		
		for id in self.bodies.keys() {
			let (body, e) = self.get_item(id).unwrap();
			let body = body.borrow();
			let iso = *body.position();
			e.render(r, iso);
		}
		
		r.draw_str(&format!("{} FPS", fps), 10.0, 10.0, FONT_SIZE);
		
		r.swap();
	}
}
