use std::convert::{Into, From};

use cgmath::{vec3, Vector3};

#[derive(Copy, Clone)]
pub struct Color {
	r: f32,
	g: f32,
	b: f32,
}
impl Color {
	pub fn new(r: f32, g: f32, b: f32) -> Color {
		Color {
			r: r,
			g: g,
			b: b,
		}
	}
}

impl From<(f32, f32, f32)> for Color {
	fn from(c: (f32, f32, f32)) -> Color {
		Color::new(c.0, c.1, c.2)
	}
}

impl Into<Vector3<f32>> for Color {
	fn into(self) -> Vector3<f32> {
		vec3(self.r, self.g, self.b)
	}
}
