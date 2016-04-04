use std::rc::Rc;

use cgmath::*;

use render::{Render, RenderableMesh};
use collision::Aabb;

#[derive(Clone)]
pub struct Entity {
	/// If the entity can move. (static)
	stat: bool,
	/// Position
	pos: Vector3<f32>,
	/// Velocity
	vel: Vector3<f32>,
	/// Weight of the entity
	weight: f32,
	/// Visible mesh
	mesh: Rc<RenderableMesh>,
	/// Bounding Box when the entity is at 0,0 with scale 1.0 and no rotation.
	bb: Aabb,
	/// Transform
	trans: Decomposed<Vector3<f32>, Quaternion<f32>>,
}
impl Entity {
	pub fn new(pos: Vector3<f32>, vel: Vector3<f32>, weight: f32, mesh: Rc<RenderableMesh>, bb: Aabb, stat: bool) -> Entity {
		Entity {
			stat: stat,
			pos: pos,
			vel: vel,
			weight: weight,
			mesh: mesh,
			bb: bb,
			trans: Decomposed{scale:1.0, rot:Quaternion::one(), disp:pos},
		}
	}
	
	/// Returns the bounding box the entity has in it's current position and at it's current scale.
	pub fn bounding_box(&self) -> Aabb {
		self.bb.scale(self.trans.scale).translate(self.trans.disp)
	}
	
	/// Rotate the entity by a specified amount
	pub fn rotate(&mut self, rot: &Quaternion<f32>) {
		self.trans.rot.concat_self(rot);
	}
	
	/// Scale the entity by a specified amount
	pub fn scale(&mut self, scale: f32) {
		self.trans.scale *= scale;
	}
	
	/// Applies a force in a direction
	pub fn force(&mut self, f: Vector3<f32>) {
		if !self.stat {
			self.vel = self.vel + (f / self.weight);
		}
	}
	
	/// Processes a tick for the entity
	pub fn tick(&mut self, dt: f32) {
		self.pos = self.pos + self.vel * dt;
		self.trans.disp = self.pos;
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
	
	/// Renders the entity
	pub fn render(&self, r: &mut Render) {
		let model = 
			  Matrix4::from_translation(self.trans.disp)
			* Matrix4::from_scale(self.trans.scale)
			* Matrix4::from(*Basis3::from(self.trans.rot).as_ref());
		self.mesh.render(r, model);
	}
}
