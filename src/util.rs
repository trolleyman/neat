use std::time::Duration;
use std::cmp::PartialOrd;

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

pub fn clamp<T: PartialOrd>(v: T, min: T, max: T) -> T {
	if v < min {
		min
	} else if v > max {
		max
	} else {
		v
	}
}
