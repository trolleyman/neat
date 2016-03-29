use render::Render;
use render::Color;

use cgmath::Vector3;

#[derive(Clone)]
pub struct Entity {
	pos: Vector3<f32>,
	vel: Vector3<f32>,
	weight: f32,
	color: Color,
}
impl Entity {
	pub fn new(pos: Vector3<f32>, vel: Vector3<f32>, weight: f32, color: Color) -> Entity {
		Entity {
			pos: pos,
			vel: vel,
			weight: weight,
			color: color,
		}
	}

	/// Applies a force in a direction
	pub fn force(&mut self, f: Vector3<f32>) {
		self.vel = self.vel + (f / self.weight);
	}

	/// Processes a tick for the entity
	pub fn tick(&mut self, dt: f32) {
		self.pos = self.pos + self.vel * dt;
		println!("pos: {:?}, vel: {:?}", self.pos, self.vel);
	}
	
	/// Returns the position of the object in space
	pub fn pos(&self) -> Vector3<f32> {
		self.pos
	}
	
	/// Returns the velocity of the object
	pub fn vel(&self) -> Vector3<f32> {
		self.vel
	}
	
	/// Returns the weight of the object
	pub fn weight(&self) -> f32 {
		self.weight
	}
	
	pub fn render(&self, r: &mut Render) {
		r.draw_sphere(self.pos, 0.5, self.color);
	}
}
