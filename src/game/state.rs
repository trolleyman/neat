use std::rc::Rc;
use std::collections::HashMap;
use std::collections::hash_map::Keys;
use std::iter;

use glium::backend::Context;
use na::{Norm, Vec3};
use nc::shape::Ball;
use np::world::World;
use np::object::{RigidBody, RigidBodyHandle};

use game::{KeyboardState, Entity, EntityBuilder, GameState};
use render::{Camera, Render, SimpleMesh, ColoredMesh, Color};
use settings::Settings;

const FONT_SIZE: f32 = 24.0;

type EntityId = u32;

/// Gravity type of the simulation
#[derive(Copy, Clone)]
pub enum Gravity {
	/// Each object attracts each other object, scaled by a specified amount.
	Relative(f32),
	/// Each object is attracted in a constant direction
	Constant(Vec3<f32>),
}

pub struct State {
	next_free_id: EntityId,
	entities: HashMap<EntityId, Entity>,
	camera: Camera,
	gravity: Gravity,
	world: World<f32>,
}
impl State {
	pub fn new(cam: Camera, g: Gravity) -> State {
		State {
			next_free_id: 0,
			entities: HashMap::new(),
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
		
		let mut state = GameState::new(cam, Gravity::Relative(1.0));
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
	
	#[allow(non_snake_case)]
	pub fn gen_solar(ctx: &Rc<Context>, cam: Camera) -> State {
		let sphere = Rc::new(SimpleMesh::sphere(ctx, 4));
		
		const PI: f32 = ::std::f32::consts::PI;
		
		const SUN_MASS: f32 = 100.0;
		const SUN_RADIUS: f32 = 1.0;
		
		const EARTH_POS: f32 = 18.0;
		const EARTH_VEL: f32 = 25.0;
		const EARTH_SCALE: f32 = 0.05;
		const EARTH_MASS: f32 = SUN_MASS * EARTH_SCALE;
		let EARTH_RADIUS: f32 = ((3.0 * EARTH_SCALE) / (4.0 * PI)).cbrt();
		
		const MERCURY_POS: f32 = 10.0;
		const MERCURY_VEL: f32 = 30.0;
		const MERCURY_SCALE: f32 = 0.0005;
		const MERCURY_MASS: f32 = SUN_MASS * MERCURY_SCALE;
		let MERCURY_RADIUS: f32 = ((3.0 * MERCURY_SCALE) / (4.0 * PI)).cbrt();
		
		// Equalize forces
		const SUN_VEL: f32 = 0.45;
		
		info!("SUN    : vel: {:6.2}, scale: {:.4}, mass: {:6.2}, radius: {:.4}", SUN_VEL, 1.0, SUN_MASS, SUN_RADIUS);
		info!("EARTH  : vel: {:6.2}, scale: {:.4}, mass: {:6.2}, radius: {:.4}", EARTH_VEL, EARTH_SCALE, EARTH_MASS, EARTH_RADIUS);
		info!("MERCURY: vel: {:6.2}, scale: {:.4}, mass: {:6.2}, radius: {:.4}", MERCURY_VEL, MERCURY_SCALE, MERCURY_MASS, MERCURY_RADIUS);
		
		let yellow = box ColoredMesh::with_scale(sphere.clone(), Color::YELLOW, SUN_RADIUS);
		let green  = box ColoredMesh::with_scale(sphere.clone(), Color::GREEN , EARTH_RADIUS);
		let red    = box ColoredMesh::with_scale(sphere.clone(), Color::RED   , MERCURY_RADIUS);
		
		let mut state = GameState::new(cam, Gravity::Relative(0.007));
		let sun  = Entity::new(yellow, SUN_MASS);
		let sun = state.add_entity(RigidBody::new_dynamic(Ball::new(SUN_RADIUS), 1.0, 1.0, 0.0), box sun);
		state.get_body(&sun).unwrap().borrow_mut().set_translation(Vec3::new(0.0, 0.0, 0.0));
		state.get_body(&sun).unwrap().borrow_mut().set_lin_vel(Vec3::new(0.0, 0.0, SUN_VEL));
		let earth = Entity::new(green, EARTH_MASS);
		let earth = state.add_entity(RigidBody::new_dynamic(Ball::new(EARTH_RADIUS), 1.0, 1.0, 0.0), box earth);
		state.get_body(&earth).unwrap().borrow_mut().set_translation(Vec3::new(EARTH_POS, 0.0, 0.0));
		state.get_body(&earth).unwrap().borrow_mut().set_lin_vel(Vec3::new(0.0, 0.0, -EARTH_VEL));
		let mercury = Entity::new(red, MERCURY_MASS);
		let mercury = state.add_entity(RigidBody::new_dynamic(Ball::new(MERCURY_RADIUS), 1.0, 1.0, 0.0), box mercury);
		state.get_body(&mercury).unwrap().borrow_mut().set_translation(Vec3::new(MERCURY_POS, 0.0, 0.0));
		state.get_body(&mercury).unwrap().borrow_mut().set_lin_vel(Vec3::new(0.0, 0.0, -MERCURY_VEL));
		
		state
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
	
	/// Gets the entity with the specified id
	pub fn get_entity<'a>(&'a self, id: &EntityId) -> Option<&'a Entity> {
		self.entities.get(id)
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
				Gravity::Relative(g) => {
					let mut ids = self.bodies.keys().cloned();
					loop {
						let a_id = match ids.next() {
							Some(a) => a,
							None => break,
						};
						for b_id in ids.clone() {
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
							let f = v * ((g * a_ent.mass() * b_ent.mass()) / len_sq);
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
			e.render(r);
		}
		
		r.draw_str(&format!("{} FPS", fps), 10.0, 10.0, FONT_SIZE);
		
		r.swap();
	}
}
