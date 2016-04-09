use std::rc::Rc;
use std::collections::HashMap;

use glium::backend::Context;
use glutin::{ElementState, VirtualKeyCode};
use na::{Norm, Vec3};
use nc::shape::{Ball, Cuboid};
use np::world::World;
use np::object::RigidBody;

use game::{KeyboardState, Entity, EntityBuilder, GameState, Component};
use render::{Camera, Render, SimpleMesh, ColoredMesh, Color};
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

pub struct State {
	next_free_id: EntityId,
	entities: HashMap<EntityId, Entity>,
	camera: Camera,
	wireframe_mode: bool,
	gravity: Gravity,
	world: World<f32>,
}
impl State {
	pub fn new(cam: Camera, g: Gravity) -> State {
		State {
			next_free_id: 0,
			entities: HashMap::new(),
			camera: cam,
			wireframe_mode: false,
			gravity: g,
			world: World::new(),
		}
	}
	
	pub fn gen_balls(ctx: &Rc<Context>) -> State {
		let sphere = Rc::new(SimpleMesh::sphere(ctx, 4));
		
		let red   = Rc::new(ColoredMesh::new(sphere.clone(), Color::RED));
		let green = Rc::new(ColoredMesh::new(sphere.clone(), Color::GREEN));
		let blue  = Rc::new(ColoredMesh::new(sphere.clone(), Color::BLUE));
		
		let mut state = GameState::new(Camera::new(Vec3::new(2.0, 2.0, 10.0)), Gravity::Relative(1.0));
		EntityBuilder::new(Component::new(
			RigidBody::new_dynamic(Ball::new(1.0), 1.0, 0.9, 0.1), red))
				.pos(Vec3::new(5.0, 0.0,  0.0))
				.vel(Vec3::new(0.0, 1.0, -1.0))
				.build(&mut state);
		
		EntityBuilder::new(Component::new(
			RigidBody::new_dynamic(Ball::new(1.0), 1.0, 0.9, 0.1), green))
				.pos(Vec3::new(0.0, 0.0, -5.0))
				.vel(Vec3::new(1.0, -1.0, 1.0))
				.build(&mut state);
		
		EntityBuilder::new(Component::new(
			RigidBody::new_dynamic(Ball::new(1.0), 1.0, 0.9, 0.1), blue))
				.pos(Vec3::new(0.0, 5.0,  0.0))
				.vel(Vec3::new(-1.0, 1.0, 1.0))
				.build(&mut state);
		state
	}
	
	#[allow(non_snake_case)]
	pub fn gen_solar(ctx: &Rc<Context>) -> State {
		let sphere = Rc::new(SimpleMesh::sphere(ctx, 4));
		
		const PI: f32 = ::std::f32::consts::PI;
		
		const SUN_POS: f32 = 0.0;
		const SUN_MASS: f32 = 100.0;
		const SUN_RADIUS: f32 = 1.0;
		let SUN_VOLUME: f32 = (4.0 * PI * SUN_RADIUS * SUN_RADIUS * SUN_RADIUS) / 3.0;
		let DENSITY: f32 = SUN_MASS / SUN_VOLUME;
		
		const EARTH_POS: f32 = 18.0;
		const EARTH_VEL: f32 = 22.0;
		const EARTH_SCALE: f32 = 0.05;
		let EARTH_RADIUS: f32 = ((3.0 * EARTH_SCALE) / (4.0 * PI)).cbrt();
		
		const MERCURY_POS: f32 = 10.0;
		const MERCURY_VEL: f32 = 30.0;
		const MERCURY_SCALE: f32 = 0.0005;
		let MERCURY_RADIUS: f32 = ((3.0 * MERCURY_SCALE) / (4.0 * PI)).cbrt();
		
		// Equalize forces
		const SUN_VEL: f32 = 0.38;
		
		let yellow = Rc::new(ColoredMesh::with_scale(sphere.clone(), Color::YELLOW, SUN_RADIUS));
		let green  = Rc::new(ColoredMesh::with_scale(sphere.clone(), Color::GREEN , EARTH_RADIUS));
		let red    = Rc::new(ColoredMesh::with_scale(sphere.clone(), Color::RED   , MERCURY_RADIUS));
		
		let mut state = GameState::new(Camera::new(Vec3::new(0.0, 0.0, 20.0)), Gravity::Relative(1.0));
		let sun     = EntityBuilder::new(Component::new(
			RigidBody::new_dynamic(Ball::new(SUN_RADIUS), DENSITY, 1.0, 0.0), yellow))
				.pos(Vec3::new(SUN_POS, 0.0, 0.0))
				.vel(Vec3::new(0.0, 0.0, SUN_VEL))
				.build(&mut state);
		
		let earth   = EntityBuilder::new(Component::new(
			RigidBody::new_dynamic(Ball::new(EARTH_RADIUS), DENSITY, 1.0, 0.0), green))
				.pos(Vec3::new(EARTH_POS, 0.0, 0.0))
				.vel(Vec3::new(0.0, 0.0, -EARTH_VEL))
				.build(&mut state);
		
		let mercury = EntityBuilder::new(Component::new(
			RigidBody::new_dynamic(Ball::new(MERCURY_RADIUS), DENSITY, 1.0, 0.0), red))
				.pos(Vec3::new(MERCURY_POS, 0.0, 0.0))
				.vel(Vec3::new(0.0, 0.0, -MERCURY_VEL))
				.build(&mut state);
		
		info!("SUN    : vel: {:6.2}, scale: {:.4}, mass: {:6.2}, radius: {:.4}",
			SUN_VEL,
			1.0,
			state.get_entity(&sun).unwrap().mass(),
			SUN_RADIUS);
		info!("EARTH  : vel: {:6.2}, scale: {:.4}, mass: {:6.2}, radius: {:.4}",
			EARTH_VEL,
			EARTH_SCALE,
			state.get_entity(&earth).unwrap().mass(),
			EARTH_RADIUS);
		info!("MERCURY: vel: {:6.2}, scale: {:.4}, mass: {:6.2}, radius: {:.4}",
			MERCURY_VEL,
			MERCURY_SCALE,
			state.get_entity(&mercury).unwrap().mass(),
			MERCURY_RADIUS);
		
		state
	}
	
	pub fn gen_rot_test(ctx: &Rc<Context>) -> State {
		let sphere = Rc::new(SimpleMesh::sphere(ctx, 0));
		
		let red   = Rc::new(ColoredMesh::new(sphere.clone(), Color::RED));
		let green = Rc::new(ColoredMesh::new(sphere.clone(), Color::GREEN));
		let blue  = Rc::new(ColoredMesh::new(sphere.clone(), Color::BLUE));
		
		let mut state = State::new(Camera::new(Vec3::new(2.0, 0.0, 10.0)), Gravity::None);
		
		EntityBuilder::new(Component::new(
			RigidBody::new_dynamic(Ball::new(1.0), 1.0, 0.9, 0.1), red.clone()))
				.pos(Vec3::new(0.0, 0.0,  0.0))
				.rot(Vec3::new(0.0, 0.0, 0.0))
				.build(&mut state);
		
		EntityBuilder::new(Component::new(
			RigidBody::new_dynamic(Ball::new(1.0), 1.0, 0.9, 0.1), green.clone()))
				.pos(Vec3::new(3.0, 0.0, 0.0))
				.rot(Vec3::new(0.0, 0.0, 0.0))
				.ang_vel(Vec3::new(1.0, 0.0, 0.0))
				.build(&mut state);
		
		EntityBuilder::new(Component::new(
			RigidBody::new_dynamic(Ball::new(1.0), 1.0, 0.9, 0.1), blue.clone()))
				.pos(Vec3::new(6.0, 0.0,  0.0))
				.rot(Vec3::new(0.0, 0.0, 0.0))
				.ang_vel(Vec3::new(2.0, 0.0, 0.0))
				.build(&mut state);
		
		EntityBuilder::new(Component::new(
			RigidBody::new_dynamic(Ball::new(1.0), 1.0, 0.9, 0.1), green.clone()))
				.pos(Vec3::new(0.0, 3.0, 0.0))
				.rot(Vec3::new(0.0, 0.0, 0.0))
				.ang_vel(Vec3::new(0.0, 1.0, 0.0))
				.build(&mut state);
		
		EntityBuilder::new(Component::new(
			RigidBody::new_dynamic(Ball::new(1.0), 1.0, 0.9, 0.1), blue.clone()))
				.pos(Vec3::new(0.0, 6.0,  0.0))
				.rot(Vec3::new(0.0, 0.0, 0.0))
				.ang_vel(Vec3::new(0.0, 2.0, 0.0))
				.build(&mut state);
		
		EntityBuilder::new(Component::new(
			RigidBody::new_dynamic(Ball::new(1.0), 1.0, 0.9, 0.1), green.clone()))
				.pos(Vec3::new(0.0, 0.0, 3.0))
				.rot(Vec3::new(0.0, 0.0, 0.0))
				.ang_vel(Vec3::new(0.0, 0.0, 1.0))
				.build(&mut state);
		
		EntityBuilder::new(Component::new(
			RigidBody::new_dynamic(Ball::new(1.0), 1.0, 0.9, 0.1), blue.clone()))
				.pos(Vec3::new(0.0, 0.0,  6.0))
				.rot(Vec3::new(0.0, 0.0, 0.0))
				.ang_vel(Vec3::new(0.0, 0.0, 2.0))
				.build(&mut state);
		
		state
	}
	
	
	pub fn gen_ball_upside_down_pyramid(ctx: &Rc<Context>) -> State {
		// Gen planes
		let mut state = State::new(Camera::new(Vec3::new(0.0, 10.0, 15.0)), Gravity::Constant(Vec3::new(0.0, -9.81, 0.0)));
		
		const ANG: f32 = 0.5;
		
		let he = Vec3::new(20.0, 1.0, 20.0);
		let plane_mesh = Rc::new(SimpleMesh::cuboid(ctx, he));
		let green = Rc::new(ColoredMesh::new(plane_mesh.clone(), Color::GREEN));
		let blue  = Rc::new(ColoredMesh::new(plane_mesh.clone(), Color::BLUE));
		let plane_body = RigidBody::new_static(Cuboid::new(he), 0.1, 0.5);
		// Plane +X
		EntityBuilder::new(Component::new(plane_body.clone(), green.clone()))
			.rot(Vec3::new(0.0, 0.0, -ANG)).build(&mut state);
		// Plane +Z
		EntityBuilder::new(Component::new(plane_body.clone(), blue .clone()))
			.rot(Vec3::new(-ANG, 0.0, 0.0)).build(&mut state);
		// Plane -X
		EntityBuilder::new(Component::new(plane_body.clone(), green.clone()))
			.rot(Vec3::new(0.0, 0.0, ANG)).build(&mut state);
		// Plane -Y
		EntityBuilder::new(Component::new(plane_body.clone(), blue .clone()))
			.rot(Vec3::new(ANG, 0.0, 0.0)).build(&mut state);
		
		// Gen balls at top
		const SCALE: f32 = 0.4;
		let ball_body = RigidBody::new_dynamic(Ball::new(SCALE), 1.0, 0.3, 0.5);
		let ball_mesh = Rc::new(SimpleMesh::sphere(ctx, 4));
		const N: i32 = 10;
		for x in 0..N {
			let x = (x - N/2) as f32 * 2.0;
			for z in 0..N {
				let z = (z - N/2) as f32 * 2.0;
				let col = Color::new(1.0, 0.0, 0.0);
				EntityBuilder::new(Component::new(ball_body.clone(), Rc::new(ColoredMesh::with_scale(ball_mesh.clone(), col, SCALE)))).pos(Vec3::new(x, 20.0, z)).build(&mut state);
			}
		}
		
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
	
	pub fn calculate_gravity(&mut self, g: f32) {
		let mut ids = self.entities.keys().cloned();
		
		loop {
			let b_ids = ids.clone();
			let a_i = match ids.next() {
				Some(i) => i,
				None => break,
			};
			//info!("a_i:{}", a_i);
			for a_j in 0..self.get_entity(&a_i).unwrap().components().len() {
				//info!(" a_j:{}", a_j);
				for b_i in b_ids.clone() {
					//info!("  b_i:{}", b_i);
					let start = if a_i == b_i {
						a_j + 1
					} else {
						0
					};
					for b_j in start..self.get_entity(&b_i).unwrap().components().len() {
						//info!("   b_j:{}", b_j);
						let mut a = self.get_entity(&a_i)
							.unwrap()
							.components()[a_j]
							.body()
							.borrow_mut();
						let mut b = self.get_entity(&b_i)
							.unwrap()
							.components()[b_j]
							.body()
							.borrow_mut();
							
						let a_mass = match a.mass() { Some(m) => m, None => continue };
						let b_mass = match b.mass() { Some(m) => m, None => continue };
						
						// Get unit vector from a to b
						let mut v = b.position().translation - a.position().translation;
						let len_sq = v.sqnorm();
						v = v / len_sq.sqrt();
						
						// Calc && apply the force.
						let f = v * ((g * a_mass * b_mass) / len_sq);
						a.apply_central_impulse(f);
						b.apply_central_impulse(-f);
					}
				}
			}
		}
	}

	pub fn render(&mut self, r: &mut Render, fps: u32) {
		r.set_camera(self.camera);
		r.set_wireframe_mode(self.wireframe_mode);
		
		for e in self.entities.values() {
			e.render(r);
		}
		
		r.draw_str(&format!("{} FPS", fps), 10.0, 10.0, FONT_SIZE);
		
		r.swap();
	}
}
