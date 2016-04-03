use cgmath::*;

/// Holds data on the collision between two objects.
pub struct Collision {
	// The point of the collision
	pub point : Vector3<f32>,
	// The normal of the plane of collision, pointing towards the first object.
	pub normal: Vector3<f32>,
}
impl Collision {
	pub fn new(point: Vector3<f32>, normal: Vector3<f32>) -> Collision {
		Collision {
			point: point,
			normal: normal,
		}
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
			return None; // Sphere is inside other sphere.
		}
		
		let normal = diff.normalize();
		let point = diff * (s2.radius / (s1.radius + s2.radius));
		
		Some(Collision::new(point, normal))
	}
}

pub struct Sphere {
	pub centre: Vector3<f32>,
	pub radius: f32,
}
pub struct InfinitePlane {
	pub point: Vector3<f32>,
	pub normal: Vector3<f32>,
}

/// A 'collision mesh', or, how the object looks to the collision detection system.
pub enum Collider {
	Sphere(Sphere),
	//InfinitePlane(InfinitePlane),
}

impl Collider {
	pub fn intersection(&self, other: &Collider) -> Option<Collision> {
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
		assert!(Collision::sphere_sphere(&Sphere{centre:vec3(0.0, 0.0, 0.0), radius:10.0}, &Sphere{centre:vec3(0.0, 5.0, 0.0), radius:2.0}).is_none()); // Inside
	}
}
