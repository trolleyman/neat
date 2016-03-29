//! TODO: impl Iterator and IntoIterator for Vec3
use std::convert::From;
use std::ops::{Add, Div, Mul, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

#[derive(Copy, Clone, Default)]
pub struct Vec3 {
	pub x: f64,
	pub y: f64,
	pub z: f64,
}
impl Vec3 {
	pub fn new() -> Vec3 {
		Vec3::default()
	}

	pub fn xyz(x: f64, y: f64, z: f64) -> Vec3 {
		Vec3 { x: x, y: y, z: z }
	}
	
	#[inline]
	/// Returns the vector that, when applied to this vector, gives the vector `v`
	pub fn to(&self, v: Vec3) -> Vec3 {
		v - *self
	}
	
	#[inline]
	// Returns the length of this vector squared
	pub fn len_sq(&self) -> f64 {
		self.x * self.x + self.y * self.y + self.z * self.z
	}
	
	/// Returns the length of this vector
	#[inline]
	pub fn len(&self) -> f64 {
		self.len_sq().sqrt()
	}
}
impl From<(f64, f64, f64)> for Vec3 {
	fn from(v: (f64, f64, f64)) -> Vec3 {
		Vec3 {
			x: v.0,
			y: v.1,
			z: v.2,
		}
	}
}

impl Add<Vec3> for Vec3 {
	type Output = Vec3;
	fn add(self, rhs: Vec3) -> Vec3 {
		Vec3 {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
			z: self.z + rhs.z,
		}
	}
}
impl Sub<Vec3> for Vec3 {
	type Output = Vec3;
	fn sub(self, rhs: Vec3) -> Vec3 {
		Vec3 {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
			z: self.z - rhs.z,
		}
	}
}
impl Mul<Vec3> for Vec3 {
	type Output = Vec3;
	fn mul(self, rhs: Vec3) -> Vec3 {
		Vec3 {
			x: self.x * rhs.x,
			y: self.y * rhs.y,
			z: self.z * rhs.z,
		}
	}
}
impl Div<Vec3> for Vec3 {
	type Output = Vec3;
	fn div(self, rhs: Vec3) -> Vec3 {
		Vec3 {
			x: self.x / rhs.x,
			y: self.y / rhs.y,
			z: self.z / rhs.z,
		}
	}
}

impl Add<f64> for Vec3 {
	type Output = Vec3;
	fn add(self, rhs: f64) -> Vec3 {
		Vec3 {
			x: self.x + rhs,
			y: self.y + rhs,
			z: self.z + rhs,
		}
	}
}
impl Sub<f64> for Vec3 {
	type Output = Vec3;
	fn sub(self, rhs: f64) -> Vec3 {
		Vec3 {
			x: self.x - rhs,
			y: self.y - rhs,
			z: self.z - rhs,
		}
	}
}
impl Mul<f64> for Vec3 {
	type Output = Vec3;
	fn mul(self, rhs: f64) -> Vec3 {
		Vec3 {
			x: self.x * rhs,
			y: self.y * rhs,
			z: self.z * rhs,
		}
	}
}
impl Div<f64> for Vec3 {
	type Output = Vec3;
	fn div(self, rhs: f64) -> Vec3 {
		Vec3 {
			x: self.x / rhs,
			y: self.y / rhs,
			z: self.z / rhs,
		}
	}
}

impl AddAssign<Vec3> for Vec3 {
	fn add_assign(&mut self, rhs: Vec3) {
		self.x += rhs.x;
		self.y += rhs.y;
		self.z += rhs.z;
	}
}
impl SubAssign<Vec3> for Vec3 {
	fn sub_assign(&mut self, rhs: Vec3) {
		self.x -= rhs.x;
		self.y -= rhs.y;
		self.z -= rhs.z;
	}
}
impl MulAssign<Vec3> for Vec3 {
	fn mul_assign(&mut self, rhs: Vec3) {
		self.x *= rhs.x;
		self.y *= rhs.y;
		self.z *= rhs.z;
	}
}
impl DivAssign<Vec3> for Vec3 {
	fn div_assign(&mut self, rhs: Vec3) {
		self.x /= rhs.x;
		self.y /= rhs.y;
		self.z /= rhs.z;
	}
}

impl AddAssign<f64> for Vec3 {
	fn add_assign(&mut self, rhs: f64) {
		self.x += rhs;
		self.y += rhs;
		self.z += rhs;
	}
}
impl SubAssign<f64> for Vec3 {
	fn sub_assign(&mut self, rhs: f64) {
		self.x -= rhs;
		self.y -= rhs;
		self.z -= rhs;
	}
}
impl MulAssign<f64> for Vec3 {
	fn mul_assign(&mut self, rhs: f64) {
		self.x *= rhs;
		self.y *= rhs;
		self.z *= rhs;
	}
}
impl DivAssign<f64> for Vec3 {
	fn div_assign(&mut self, rhs: f64) {
		self.x /= rhs;
		self.y /= rhs;
		self.z /= rhs;
	}
}
