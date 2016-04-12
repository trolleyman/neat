use prelude::*;
use std::cmp::PartialOrd;

pub fn clamp<T: PartialOrd>(v: T, min: T, max: T) -> T {
	if v > min {
		if v < max {
			v
		} else {
			max
		}
	} else {
		min
	}
}

pub fn min<T: PartialOrd>(v1: T, v2: T) -> T {
	if v1 < v2 {
		v1
	} else {
		v2
	}
}

pub fn max<T: PartialOrd>(v1: T, v2: T) -> T {
	if v1 > v2 {
		v1
	} else {
		v2
	}
}

/// Lerps some vectors
pub fn lerp(a: Vec3<f32>, b: Vec3<f32>, s: f32) -> Vec3<f32> {
	let ab = b - a;
	a + ab * s
}

/// Converts an angle from degrees to radians
pub fn to_rad(angle_degrees: f32) -> f32 {
	angle_degrees / 180.0 * ::std::f32::consts::PI
}

/// Creates a 4x4 matrix from a non-uniform scale
pub fn mat4_scale(s: Vec3<f32>) -> Mat4<f32> {
	Mat4::new(
		s.x,0.0,0.0,0.0,
		0.0,s.y,0.0,0.0,
		0.0,0.0,s.z,0.0,
		0.0,0.0,0.0,1.0,
		)
}

pub fn mat4_translation(t: Vec3<f32>) -> Mat4<f32> {
	Mat4::new(
		1.0,0.0,0.0,t.x,
		0.0,1.0,0.0,t.y,
		0.0,0.0,1.0,t.z,
		0.0,0.0,0.0,1.0,
		)
}

#[allow(dead_code)]
fn mat4_to_string(m: Mat4<f32>) -> String {
	let mut s = String::new();
	for (i, col) in m.as_ref().iter().enumerate() {
		s.push(if i == 0 { '[' } else { ' ' });
		s.push_str(&format!("{:?}", col));
		s.push_str(if i == 3 { "]" } else { ",\n" });
	}
	s
}

#[cfg(test)]
mod tests {
	use super::*;
	use na::{Vec3, Vec4, FromHomogeneous};
	
	#[test]
	pub fn test_mat4_scale() {
		let mat = mat4_scale(Vec3::new(1.0, 2.0, 3.0));
		let v = Vec3::new(1.0, 10.0, -100.0);
		assert_eq!(Vec3::new(1.0, 20.0, -300.0), <Vec3<f32> as FromHomogeneous<Vec4<f32>>>::from(&(mat * Vec4::new(v.x, v.y, v.z, 1.0))));
	}
	
	#[test]
	pub fn test_mat4_translation() {
		let mat = mat4_translation(Vec3::new(1.0, -2.0, 3.0));
		let v = Vec3::new(1.0, 10.0, -100.0);
		let ret = mat * Vec4::new(v.x, v.y, v.z, 1.0);
		let ret = Vec3::new(ret.x, ret.y, ret.z) * ret.w;
		assert_eq!(Vec3::new(2.0, 8.0, -97.0), ret);
	}
}
