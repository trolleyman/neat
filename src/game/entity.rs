use std::rc::Rc;

use na::{Vec3, Sim3, Eye, Rot3, Rotation, Translation, ToHomogeneous};

use render::{Render, RenderableMesh};

#[derive(Clone)]
pub struct Entity {
	/// If the entity can move. (static)
	stat: bool,
	/// Position
	pos: Vec3<f32>,
	/// Velocity
	vel: Vec3<f32>,
	/// Weight of the entity
	weight: f32,
	/// Visible mesh
	mesh: Rc<RenderableMesh>,
	/// Transform
	trans: Sim3<f32>,
}
impl Entity {
	pub fn new(pos: Vec3<f32>, vel: Vec3<f32>, weight: f32, mesh: Rc<RenderableMesh>, stat: bool) -> Entity {
		Entity {
			stat: stat,
			pos: pos,
			vel: vel,
			weight: weight,
			mesh: mesh,
			trans: Sim3::new_with_rotmat(pos, Rot3::new_identity(3), 1.0),
		}
	}
	
	/// Rotate the entity by a specified amount (axis-angle format)
	pub fn rotate(&mut self, rot: Vec3<f32>) {
		self.trans.isometry.append_rotation_mut(&rot);
	}
	
	/// Scale the entity by a specified amount
	pub fn scale(&mut self, scale: f32) {
		self.trans.append_scale_mut(&scale);
	}
	
	/// Applies a force in a direction
	pub fn force(&mut self, f: Vec3<f32>) {
		if !self.stat {
			self.vel = self.vel + (f / self.weight);
		}
	}
	
	/// Processes a tick for the entity
	pub fn tick(&mut self, dt: f32) {
		self.pos = self.pos + self.vel * dt;
		self.trans.isometry.set_translation(self.pos);
	}
	
	/// Returns the position of the object in space
	pub fn pos(&self) -> Vec3<f32> {
		self.pos
	}
	
	/// Returns the velocity of the object
	pub fn vel(&self) -> Vec3<f32> {
		self.vel
	}
	
	/// Returns the weight of the object
	pub fn weight(&self) -> f32 {
		self.weight
	}
	
	/// Renders the entity
	pub fn render(&self, r: &mut Render) {
		self.mesh.render(r, self.trans.to_homogeneous());
	}
}
