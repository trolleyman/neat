use std::convert::Into;

use render::Render;
use render::Color;
use math::Vec3;

#[derive(Clone)]
pub struct Entity {
	pos: Vec3,
	vel: Vec3,
	weight: f64,
	color: Color,
}
impl Entity {
	pub fn new<V: Into<Vec3>, C: Into<Color>>(pos: V, vel: V, weight: f64, color: C) -> Entity {
		Entity {
			pos: pos.into(),
			vel: vel.into(),
			weight: weight,
			color: color.into(),
		}
	}

	/// Applies a force in a direction
	pub fn force<T: Into<Vec3>>(&mut self, f: T) {
		self.vel += f.into() / self.weight;
	}

	/// Processes a tick for the entity
	pub fn tick(&mut self, dt: f64) {
		self.pos = self.vel * dt;
	}
	
	/// Returns the position of the object in space
	pub fn pos(&self) -> Vec3 {
		self.pos
	}
	
	/// Returns the velocity of the object
	pub fn vel(&self) -> Vec3 {
		self.vel
	}
	
	/// Returns the weight of the object
	pub fn weight(&self) -> f64 {
		self.weight
	}
	
	pub fn render(&self, r: &mut Render) {
		r.draw_sphere(self.pos, 0.5, self.color);
	}
}
