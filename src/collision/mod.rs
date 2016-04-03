use cgmath::*;

/// Holds data on the collision between two objects.
#[derive(Copy, Clone)]
pub struct Collision {
	/// The point of the collision
	pub point : Vector3<f32>,
	/// The normal of the plane of collision, pointing towards the first object.
	pub normal: Vector3<f32>,
	/// The force to apply to both objects to resolve the collision.
	pub impulse: Vector3<f32>,
}
impl Collision {
	pub fn new(point: Vector3<f32>, normal: Vector3<f32>, impulse: Vector3<f32>) -> Collision {
		Collision {
			point  : point,
			normal : normal,
			impulse: impulse,
		}
	}
	
	/// Invert the normal and impulse of the collision
	pub fn invert(&mut self) {
		self.normal  = self.normal  * -1.0;
		self.impulse = self.impulse * -1.0;
	}
	
	pub fn sphere_sphere(s1: &Sphere, s2: &Sphere) -> Option<Collision> {
		// Get avg of centres
		use util::{min, max};
		
		let diff = s2.centre - s1.centre;
		let dist = diff.length();
		if dist > s1.radius + s2.radius {
			return None;
		}
		if dist < max(s1.radius, s2.radius) - min(s1.radius, s2.radius) {
			// Sphere is inside other sphere.
			// TODO: Fix
			return None;
		}
		
		let normal = diff.normalize();
		let point = diff * (s2.radius / (s1.radius + s2.radius));
		let impulse = Vector3::zero();
		
		Some(Collision::new(point, normal, impulse))
	}
}

#[derive(Copy, Clone)]
pub struct Sphere {
	pub centre: Vector3<f32>,
	pub radius: f32,
}
impl Sphere {
	pub fn transformed(self, trans: Decomposed<Vector3<f32>, Quaternion<f32>>) -> Sphere {
		Sphere {
			centre: trans.transform_vector(self.centre),
			radius: trans.scale * self.radius,
		}
	}
}
pub struct InfinitePlane {
	pub point: Vector3<f32>,
	pub normal: Vector3<f32>,
}

/// A 'collision mesh', or, how the object looks to the collision detection system.
#[derive(Copy, Clone)]
pub enum Collider {
	Sphere(Sphere),
	//InfinitePlane(InfinitePlane),
}

impl Collider {
	pub fn transformed(self, trans: Decomposed<Vector3<f32>, Quaternion<f32>>) -> Collider {
		use self::Collider::*;
		
		match self {
			Sphere(s) => Sphere(s.transformed(trans)),
		}
	}
	
	pub fn sphere(centre: Vector3<f32>, radius: f32) -> Collider {
		Collider::Sphere(Sphere{
			centre: centre,
			radius: radius,
		})
	}
	
	pub fn collision(&self, other: &Collider) -> Option<Collision> {
		use self::Collider::*;
		
		match (self, other) {
			(&Sphere(ref s1), &Sphere(ref s2)) => Collision::sphere_sphere(s1, s2),
		}
	}
}

#[cfg(test)]
mod test {
	#[test]
	fn test_sphere_sphere() {
		use super::*;
		use cgmath::*;
		
		assert!(Collision::sphere_sphere(&Sphere{centre:vec3(0.0, 0.0, 0.0), radius:10.0}, &Sphere{centre:vec3(0.0, 13.0, 0.0), radius:2.0}).is_none()); // Outside
		assert!(Collision::sphere_sphere(&Sphere{centre:vec3(0.0, 0.0, 0.0), radius:10.0}, &Sphere{centre:vec3(0.0, 10.0, 0.0), radius:2.0}).is_some()); // Intersection
		assert!(Collision::sphere_sphere(&Sphere{centre:vec3(0.0, 0.0, 0.0), radius:10.0}, &Sphere{centre:vec3(0.0, 5.0, 0.0), radius:2.0}).is_some()); // Inside
	}
}
