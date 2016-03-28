//! TODO: impl Iterator and IntoIterator for Vec3
use std::convert::From;
use std::ops::{Add, Sub, Mul, Div};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};

#[derive(Copy, Clone, Default)]
pub struct Vec3 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}
impl Vec3 {
	pub fn new() -> Vec3 {
		Vec3::default()
	}
	
	pub fn xyz(x: f32, y: f32, z: f32) -> Vec3 {
		Vec3 {
			x: x,
			y: y,
			z: z,
		}
	}
}
impl From<(f32, f32, f32)> for Vec3 {
	fn from(v: (f32, f32, f32)) -> Vec3 {
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

impl Add<f32> for Vec3 {
	type Output = Vec3;
	fn add(self, rhs: f32) -> Vec3 {
		Vec3 {
			x: self.x + rhs,
			y: self.y + rhs,
			z: self.z + rhs,
		}
	}
}
impl Sub<f32> for Vec3 {
	type Output = Vec3;
	fn sub(self, rhs: f32) -> Vec3 {
		Vec3 {
			x: self.x - rhs,
			y: self.y - rhs,
			z: self.z - rhs,
		}
	}
}
impl Mul<f32> for Vec3 {
	type Output = Vec3;
	fn mul(self, rhs: f32) -> Vec3 {
		Vec3 {
			x: self.x * rhs,
			y: self.y * rhs,
			z: self.z * rhs,
		}
	}
}
impl Div<f32> for Vec3 {
	type Output = Vec3;
	fn div(self, rhs: f32) -> Vec3 {
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

impl AddAssign<f32> for Vec3 {
	fn add_assign(&mut self, rhs: f32) {
		self.x += rhs;
		self.y += rhs;
		self.z += rhs;
	}
}
impl SubAssign<f32> for Vec3 {
	fn sub_assign(&mut self, rhs: f32) {
		self.x -= rhs;
		self.y -= rhs;
		self.z -= rhs;
	}
}
impl MulAssign<f32> for Vec3 {
	fn mul_assign(&mut self, rhs: f32) {
		self.x *= rhs;
		self.y *= rhs;
		self.z *= rhs;
	}
}
impl DivAssign<f32> for Vec3 {
	fn div_assign(&mut self, rhs: f32) {
		self.x /= rhs;
		self.y /= rhs;
		self.z /= rhs;
	}
}
