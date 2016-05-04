//! Utility functions
use prelude::*;

/// Linearly interpolate `a` and `b`.
pub fn lerp(a: Vector3<f32>, b: Vector3<f32>, s: f32) -> Vector3<f32> {
	let ab = b - a;
	a + ab * s
}

/// Converts an angle from degrees to radians.
pub fn to_rad(angle_degrees: f32) -> f32 {
	angle_degrees / 180.0 * ::std::f32::consts::PI
}

/// Creates a 4x4 matrix from a non-uniform scale.
pub fn mat4_scale(s: Vector3<f32>) -> Matrix4<f32> {
	Matrix4::new(
		s.x,0.0,0.0,0.0,
		0.0,s.y,0.0,0.0,
		0.0,0.0,s.z,0.0,
		0.0,0.0,0.0,1.0,
		)
}
/// Creates a 4x4 matrix from a translation.
pub fn mat4_translation(t: Vector3<f32>) -> Matrix4<f32> {
	Matrix4::new(
		1.0,0.0,0.0,t.x,
		0.0,1.0,0.0,t.y,
		0.0,0.0,1.0,t.z,
		0.0,0.0,0.0,1.0,
		)
}

/// Gets the upper-left part of a 4x4 matrix as a 3x3 matrix.
pub fn mat4_upper_left(v: Matrix4<f32>) -> Matrix3<f32> {
	Matrix3::new(
		v.m11, v.m12, v.m13,
		v.m21, v.m22, v.m23,
		v.m31, v.m32, v.m33,
		)
}

/// Converts a 4x4 matrix into a human-readable string.
#[allow(dead_code)]
fn mat4_to_string(m: Matrix4<f32>) -> String {
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
	use na::{Vector3, Vector4, FromHomogeneous};
	
	#[test]
	pub fn test_mat4_scale() {
		let mat = mat4_scale(Vector3::new(1.0, 2.0, 3.0));
		let v = Vector3::new(1.0, 10.0, -100.0);
		assert_eq!(Vector3::new(1.0, 20.0, -300.0), <Vector3<f32> as FromHomogeneous<Vector4<f32>>>::from(&(mat * Vector4::new(v.x, v.y, v.z, 1.0))));
	}
	
	#[test]
	pub fn test_mat4_translation() {
		let mat = mat4_translation(Vector3::new(1.0, -2.0, 3.0));
		let v = Vector3::new(1.0, 10.0, -100.0);
		let ret = mat * Vector4::new(v.x, v.y, v.z, 1.0);
		let ret = Vector3::new(ret.x, ret.y, ret.z) * ret.w;
		assert_eq!(Vector3::new(2.0, 8.0, -97.0), ret);
	}
}
