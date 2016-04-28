//! A prelude imported into all files because I'm lazy.
pub use std::time::{Instant, Duration};

pub use na::{Point2, Point3, Point4, Vector2, Vector3, Vector4, Matrix3, Matrix4, UnitQuaternion, Rotation3, Isometry3, Similarity3, Perspective3};
pub use na::{Eye, Inverse, Transpose, Norm, Cross, ToHomogeneous, FromHomogeneous};
pub use num::{Zero, One};
pub use glium::backend::Context;
pub use rand::Rng;

/// Helper trait extending Duration
///
/// Note: these functions should not be used when the Duration could be large, as they will be very innacurate
pub trait DurationExt {
	/// Returns the number of milliseconds in the Duration, rounded down
	fn as_millis(&self) -> u64;
	/// Returns the number of seconds in the Duration, as a floating point number
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

/// Simple stopwatch.
#[derive(Copy, Clone)]
pub struct Stopwatch {
	start: Instant,
}
impl Stopwatch {
	/// Starts a new stopwatch.
	pub fn start() -> Stopwatch {
		Stopwatch {
			start: Instant::now(),
		}
	}
	/// Returns the duration elapsed since the stopwatch was started.
	pub fn elapsed(&self) -> Duration {
		self.start.elapsed()
	}
	/// Returns the duration elapsed since the stopwatch was started, in milliseconds.
	pub fn elapsed_ms(&self) -> u64 {
		self.start.elapsed().as_millis()
	}
	/// Returns the duration elapsed since the stopwatch was started, in seconds.
	pub fn elapsed_secs(&self) -> f64 {
		self.start.elapsed().as_secs_partial()
	}
}