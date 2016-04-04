use cgmath::*;

use game::Entity;

/// Axis-aligned bounding box.
#[derive(Copy, Clone)]
pub struct Aabb {
	min: Vector3<f32>,
	max: Vector3<f32>,
}
impl Aabb {
	pub fn new(a: Vector3<f32>, b: Vector3<f32>) -> Aabb {
		use util::{min, max};
		Aabb {
			min: Vector3::new(min(a.x, b.x), min(a.y, b.y), min(a.z, b.z)),
			max: Vector3::new(max(a.x, b.x), max(a.y, b.y), max(a.z, b.z)),
		}
	}
	
	pub fn scale(self, scale: f32) -> Aabb {
		Aabb {
			min: self.min * scale,
			max: self.max * scale,
		}
	}
	pub fn translate(self, pos: Vector3<f32>) -> Aabb {
		Aabb {
			min: self.min + pos,
			max: self.max + pos,
		}
	}
}

/// Computes a collision between two entities and adds forces to ensure the collision
/// is corrected. If there is no collision, no forces are added.
pub fn calc_collision(a: &mut Entity, b: &mut Entity) {
	// TODO
}
