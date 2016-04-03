use std::time::Duration;

use cgmath::*;

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
		
		// Apply gravity to all non-static entities.
		const G: Vector3<f32> = Vector3{ x: 0.0, y: -9.81, z: 0.0};
		for e in &mut self.entities {
			if let Some(w) = e.weight() {
				e.force(G * w);
			}
		}
		
		// Collision check
		for i in 0..self.entities.len() {
			for j in i+1..self.entities.len() {
				if let Some(col) = self.entities[i].collision(&self.entities[j]) {
					let a = self.entities[i].pos();
					let b = self.entities[j].pos();
					info!("Collision: Entity at [{},{},{}] with entity at [{},{},{}]", a.x, a.y, a.z, b.x, b.y, b.z);
				}
			}
		}

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
