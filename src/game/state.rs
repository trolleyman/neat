use std::time::Duration;
use std::rc::Rc;

use glium::backend::Context;
use na::{Norm, Vec3};

use game::{KeyboardState, Entity, GameState};
use render::{Camera, Render, SimpleMesh, ColoredMesh, Color};
use settings::Settings;
use util::DurationExt;

const FONT_SIZE: f32 = 24.0;

/// Gravity type of the simulation
#[derive(Copy, Clone)]
pub enum Gravity {
	/// Each object attracts each other object
	Relative,
	/// Each object is attracted in a constant direction
	Constant(Vec3<f32>),
}

#[derive(Clone)]
pub struct State {
	entities: Vec<Entity>,
	camera: Camera,
	gravity: Gravity,
}
impl State {
	pub fn new(cam: Camera, g: Gravity) -> State {
		State {
			entities: Vec::new(),
			camera: cam,
			gravity: g,
		}
	}
	
	pub fn gen_balls(ctx: &Rc<Context>, cam: Camera) -> State {
		let sphere = Rc::new(SimpleMesh::sphere(ctx, 4));
		
		let red   = Rc::new(ColoredMesh::new(sphere.clone(), Color::RED));
		let green = Rc::new(ColoredMesh::new(sphere.clone(), Color::GREEN));
		let blue  = Rc::new(ColoredMesh::new(sphere.clone(), Color::BLUE));
		
		let mut state = GameState::new(cam, Gravity::Relative);
		state.add_entity(Entity::dynamic(Vec3::new(5.0, 0.0,  0.0), Vec3::new(0.0, 1.0, -1.0), 1.0, red));
		state.add_entity(Entity::dynamic(Vec3::new(0.0, 0.0, -5.0), Vec3::new(1.0, -1.0, 0.0), 1.0, green));
		state.add_entity(Entity::dynamic(Vec3::new(0.0, 5.0,  0.0), Vec3::new(-1.0, 0.0, 1.0), 1.0, blue));
		state
	}
	
	pub fn gen_solar(ctx: &Rc<Context>, cam: Camera) -> State {
		let sphere = Rc::new(SimpleMesh::sphere(ctx, 4));
		
		let yellow = Rc::new(ColoredMesh::new(sphere.clone(), Color::YELLOW));
		let green  = Rc::new(ColoredMesh::new(sphere.clone(), Color::GREEN));
		let red    = Rc::new(ColoredMesh::new(sphere.clone(), Color::RED));
		
		let mut state = GameState::new(cam, Gravity::Relative);
		let sun = Entity::dynamic(Vec3::new( 0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.7505), 100.0, yellow);
		state.add_entity(sun);
		
		let mut earth = Entity::dynamic(Vec3::new(10.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -35.0), 5.0, green);
		earth.set_scale(0.3684);
		state.add_entity(earth);
		
		let mut mercury = Entity::dynamic(Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -15.0), 0.05, red);
		mercury.set_scale(0.07937);
		state.add_entity(mercury);
		
		state
	}
	
	pub fn camera(&self) -> &Camera {
		&self.camera
	}
	
	pub fn entities(&self) -> &[Entity] {
		&self.entities
	}
	
	pub fn add_entity(&mut self, e: Entity) {
		self.entities.push(e);
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
			for i in 0..self.entities.len() {
				let attractor = self.entities[i].clone();
				for j in 0..self.entities.len() {
					if i == j {
						continue;
					}
					//const G: f64 = 6.674e-11;
					const G: f32 = 0.05;
					
					let mut o = &mut self.entities[j];
					// Get unit vector from o to attractor
					let mut v = attractor.pos() - o.pos();
					let len_sq = v.norm();
					v = v / len_sq.sqrt();
					
					// Apply a force towards the attractor.
					let f = v * ((G * attractor.weight() * o.weight()) / len_sq);
					o.force(f);
				}
			}
			/*const G: Vector3<f32> = Vector3{ x: 0.0, y: -9.81, z: 0.0};
			for e in &mut self.entities {
				if let Some(w) = e.weight() {
					e.force(G * w);
				}
			}*/
			
			// Collision check
			// TODO

			// Tick entities
			for e in &mut self.entities {
				e.tick(dt);
			}
		}
	}

	pub fn render(&mut self, r: &mut Render, dt: Duration) {
		r.set_camera(self.camera);
		
		for e in self.entities.iter() {
			e.render(r);
		}
		
		r.draw_str(&format!("{}ms", dt.as_millis()), 10.0, 10.0 + FONT_SIZE, FONT_SIZE);
		
		r.swap();
	}
}
