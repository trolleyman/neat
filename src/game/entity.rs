use std::convert::Into;

use math::Vec3;

#[derive(Clone, Default)]
pub struct Entity {
	pos: Vec3,
	vel: Vec3,
	weight: f32,
}
impl Entity {
	pub fn new() {
		Default::default()
	}

	/// Apply a force in a direction
	pub fn force<T: Into<Vec3>>(&mut self, f: T) {
		self.vel += f.into() / self.weight;
	}

	/// Process a tick for the entity.
	/// Update position using velocity.
	pub fn tick(&mut self, dt: f32) {
		self.pos = self.vel * dt;
	}
}
