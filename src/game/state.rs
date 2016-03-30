use glutin::VirtualKeyCode as KeyCode;

use cgmath::{EuclideanVector, Vector3};

use game::{KeyboardState, Entity};
use render::{Camera, Render};

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
	
	pub fn tick(&mut self, dt: f32, keyboard: &KeyboardState) {
		// m/s
		let speed = 2.0 * dt;
		
		// Translate camera based on keyboard state
		let mut trans = Vector3::new(0.0, 0.0, 0.0);
		if keyboard.is_pressed(&KeyCode::W) { // Forward
			trans = trans + Vector3::new(0.0, 0.0, -speed);
		}
		if keyboard.is_pressed(&KeyCode::S) { // Backward
			trans = trans + Vector3::new(0.0, 0.0,  speed);
		}
		if keyboard.is_pressed(&KeyCode::A) { // Strafe left
			trans = trans + Vector3::new(-speed, 0.0, 0.0);
		}
		if keyboard.is_pressed(&KeyCode::D) { // Strafe right
			trans = trans + Vector3::new( speed, 0.0, 0.0);
		}
		if keyboard.is_pressed(&KeyCode::Q) { // Go up
			trans = trans + Vector3::new(0.0,  speed, 0.0);
		}
		if keyboard.is_pressed(&KeyCode::E) { // Go down
			trans = trans + Vector3::new(0.0, -speed, 0.0);
		}
		self.camera.translate(trans);
		
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

	pub fn render(&self, r: &mut Render) {
		r.set_camera(self.camera);
		
		for e in self.entities.iter() {
			e.render(r);
		}
		
		r.swap();
	}
}
