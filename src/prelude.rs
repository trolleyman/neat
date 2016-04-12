pub use std::time::{Instant, Duration};

pub use na::{Pnt2, Pnt3, Pnt4, Vec2, Vec3, Vec4, Mat3, Mat4, UnitQuat, Rot3, Iso3, Sim3, Persp3};
pub use na::{Eye, Inv, Transpose, Norm, Cross, ToHomogeneous};
pub use num::{Zero, One};
pub use glium::backend::Context;
pub use rand::Rng;

pub trait DurationExt {
	fn as_millis(&self) -> u64;

	fn as_secs_partial(&self) -> f64;
}
impl DurationExt for Duration {
	fn as_millis(&self) -> u64 {
		let secs = self.as_secs();
		let subsec_millis = self.subsec_nanos() as u64 / 1_000_000;
		(secs * 1000) + subsec_millis
	}

	fn as_secs_partial(&self) -> f64 {
		self.as_secs() as f64 + (self.subsec_nanos() as f64 / 1_000_000_000.0)
	}
}