use std::convert::{Into, From};

use na::{Vec3, Vec4};

#[derive(Copy, Clone)]
pub struct Light {
	pub pos: Vec3<f32>,
	pub intensity_ambient: Vec4<f32>,
	pub intensity_specular: Vec4<f32>,
	pub intensity_diffuse: Vec4<f32>,
}
impl Light {
	pub fn new(pos: Vec3<f32>, ambient: Vec4<f32>, specular: Vec4<f32>, diffuse: Vec4<f32>) -> Light {
		Light {
			pos: pos,
			intensity_ambient: ambient,
			intensity_specular: specular,
			intensity_diffuse: diffuse,
		}
	}
	pub fn off() -> Light {
		let z = Vec4::new(0.0, 0.0, 0.0, 0.0);
		Light::new(Vec3::new(0.0, 0.0, 0.0), z, z, z)
	}
}
#[derive(Copy, Clone)]
pub struct Material {
	pub reflection_ambient: Vec4<f32>,
	pub reflection_specular: Vec4<f32>,
	pub reflection_diffuse: Vec4<f32>,
	pub shininess: f32,
}
impl Material {
	pub fn new(ambient: Vec4<f32>, specular: Vec4<f32>, diffuse: Vec4<f32>, shininess: f32) -> Material {
		Material {
			reflection_ambient: ambient,
			reflection_specular: specular,
			reflection_diffuse: diffuse,
			shininess: shininess,
		}
	}
}

#[derive(Copy, Clone, Debug)]
pub struct Color {
	r: f32,
	g: f32,
	b: f32,
}
impl Color {
	pub const BLACK : Color = Color { r: 0.0, g: 0.0, b: 0.0 };
	pub const WHITE : Color = Color { r: 1.0, g: 1.0, b: 1.0 };
	
	pub const RED   : Color = Color { r: 1.0, g: 0.0, b: 0.0 };
	pub const GREEN : Color = Color { r: 0.0, g: 1.0, b: 0.0 };
	pub const BLUE  : Color = Color { r: 0.0, g: 0.0, b: 1.0 };
	
	pub const YELLOW: Color = Color { r: 1.0, g: 1.0, b: 0.0 };
	pub const CYAN  : Color = Color { r: 0.0, g: 1.0, b: 1.0 };
	pub const PINK  : Color = Color { r: 1.0, g: 0.0, b: 1.0 };
	
	pub fn new(r: f32, g: f32, b: f32) -> Color {
		Color {
			r: r,
			g: g,
			b: b,
		}
	}
	pub fn uniform(v: f32) -> Color {
		Color::new(v, v, v)
	}
	pub fn into_array(self) -> [f32; 3] {
		self.into()
	}
	pub fn into_vec3(self) -> Vec3<f32> {
		self.into()
	}
}

impl From<[f32; 3]> for Color {
	fn from(c: [f32; 3]) -> Color {
		Color::new(c[0], c[1], c[2])
	}
}
impl From<Vec3<f32>> for Color {
	fn from(c: Vec3<f32>) -> Color {
		Color::new(c.x, c.y, c.z)
	}
}

impl Into<[f32; 3]> for Color {
	fn into(self) -> [f32; 3] {
		[self.r, self.g, self.b]
	}
}
impl Into<Vec3<f32>> for Color {
	fn into(self) -> Vec3<f32> {
		Vec3::new(self.r, self.g, self.b)
	}
}