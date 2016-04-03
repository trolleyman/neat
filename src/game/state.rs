use std::time::Duration;

use cgmath::{EuclideanVector, vec3};

use game::{KeyboardState, Entity};
use render::{Camera, Render};
use settings::Settings;
use util::DurationExt;

const FONT_SIZE: f32 = 24.0;

#[derive(Clone)]
pub struct State {
	entities: Vec<Entity>,
	camera: Camera,
}
impl State {
	pub fn new(cam: Camera) -> State {
		State {
			entities: Vec::new(),
			camera: cam,
		}
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
		let speed = 2.0 * dt;
		
		// Translate camera based on keyboard state
		let mut trans = vec3(0.0, 0.0, 0.0);
		if keyboard.is_pressed(&settings.forward) {
			trans = trans + vec3(0.0, 0.0, -speed);
		}
		if keyboard.is_pressed(&settings.backward) {
			trans = trans + vec3(0.0, 0.0,  speed);
		}
		if keyboard.is_pressed(&settings.left) {
			trans = trans + vec3(-speed, 0.0, 0.0);
		}
		if keyboard.is_pressed(&settings.right) {
			trans = trans + vec3( speed, 0.0, 0.0);
		}
		if keyboard.is_pressed(&settings.up) {
			trans = trans + vec3(0.0,  speed, 0.0);
		}
		if keyboard.is_pressed(&settings.down) {
			trans = trans + vec3(0.0, -speed, 0.0);
		}
		self.camera.translate(trans);
		if trans != vec3(0.0, 0.0, 0.0) {
			debug!("Camera moved: {:?}", trans);
		}
		
		self.camera.mouse_moved(mouse_state.0, mouse_state.1);
		
		// Apply gravity to all entities.
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
				let len_sq = v.length2();
				v = v / len_sq.sqrt();
				
				// Apply a force towards the attractor.
				let f = v * ((G * attractor.weight() * o.weight()) / len_sq);
				o.force(f);
			}
		}
		
		// Collision check
		// TODO

		// Tick entities
		for e in &mut self.entities {
			e.tick(dt);
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
