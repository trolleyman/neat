use std::rc::Rc;

use render::{Render, Color, Mesh};

use cgmath::{Vector3, Matrix4};

#[derive(Clone, Debug)]
pub struct Entity {
	pos: Vector3<f32>,
	vel: Vector3<f32>,
	weight: f32,
	color: Color,
	mesh: Rc<Mesh>,
	collision_mesh: CollisionMesh,
}
impl Entity {
	pub fn new(pos: Vector3<f32>, vel: Vector3<f32>, weight: f32, color: Color, mesh: Rc<Mesh>) -> Entity {
		Entity {
			pos: pos,
			vel: vel,
			weight: weight,
			color: color,
			mesh: mesh,
		}
	}

	/// Applies a force in a direction
	pub fn force(&mut self, f: Vector3<f32>) {
		self.vel = self.vel + (f / self.weight);
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
	pub fn weight(&self) -> f32 {
		self.weight
	}
	
	pub fn render(&self, r: &mut Render) {
		let model = Matrix4::from_translation(self.pos);
		self.mesh.render(r, model, self.color);
	}
}
