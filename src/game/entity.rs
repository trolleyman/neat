use std::rc::Rc;

use cgmath::{Vector3, Matrix4};

use render::{Render, RenderableMesh};
use collision::{Collision, Collider};

#[derive(Clone)]
pub struct Entity {
	/// Position
	pos: Vector3<f32>,
	/// Velocity
	vel: Vector3<f32>,
	/// Weight of the entity. If None, the entity is static.
	weight: Option<f32>,
	/// Visible mesh
	mesh: Rc<RenderableMesh>,
	/// Collider
	collider: Collider,
	pub scale: f32,
}
impl Entity {
	pub fn new(pos: Vector3<f32>, vel: Vector3<f32>, weight: Option<f32>, mesh: Rc<RenderableMesh>, collider: Collider) -> Entity {
		Entity {
			pos: pos,
			vel: vel,
			weight: weight,
			mesh: mesh,
			collider: collider,
			scale: 1.0,
		}
	}
	
	/// Applies a force in a direction
	pub fn force(&mut self, f: Vector3<f32>) {
		if let Some(w) = self.weight {
			self.vel = self.vel + (f / w);
		}
	}
	
	/// Processes a tick for the entity
	pub fn tick(&mut self, dt: f32) {
		self.pos = self.pos + self.vel * dt;
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
	pub fn weight(&self) -> Option<f32> {
		self.weight
	}
	
	/// Calculates if the entity has collided with `other`, and returns the collision data if it has.
	pub fn collision(&self, other: &Entity) -> Option<Collision> {
		self.collider.collision(&other.collider)
	}
	
	/// Renders the entity
	pub fn render(&self, r: &mut Render) {
		let model = Matrix4::from_translation(self.pos) * Matrix4::from_scale(self.scale);
		self.mesh.render(r, model);
	}
}
