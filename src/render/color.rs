use std::convert::{Into, From};

use cgmath::{vec3, Vector3};

#[derive(Copy, Clone, Debug)]
pub struct Color {
	r: f32,
	g: f32,
	b: f32,
}
impl Color {
	pub const RED  : Color = Color { r: 1.0, g: 0.0, b: 0.0 };
	pub const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0 };
	pub const BLUE : Color = Color { r: 0.0, g: 0.0, b: 1.0 };
	
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
