use prelude::*;

use glium::uniforms::{AsUniformValue, UniformValue};
use glium::buffer::{Buffer, BufferAny, BufferAnySlice, BufferMode};

use util;

/// Represents a light.
#[derive(Copy, Clone)]
pub struct Light {
	pub pos     : Vec4<f32>,
	pub diffuse : Vec4<f32>,
	pub specular: Vec4<f32>,
	pub constant_attenuation : f32,
	pub linear_attenuation   : f32,
	pub quadratic_attenuation: f32,
	/// in radians
	pub spot_cutoff   : f32,
	pub spot_exponent : f32,
	pub spot_direction: Vec3<f32>,
}
impl Light {
	/// Constructs a new directional light
	pub fn new_directional(dir: Vec3<f32>, diffuse: Vec4<f32>, specular: Vec4<f32>) -> Light {
		Light {
			pos: dir.to_homogeneous(),
			diffuse: diffuse,
			specular: specular,
			
			constant_attenuation : 0.0,
			linear_attenuation   : 0.0,
			quadratic_attenuation: 0.0,
			spot_cutoff: util::to_rad(180.0),
			spot_exponent: 0.0,
			spot_direction: Vec3::zero(),
		}
	}
	
	/// Constructs a new point light
	pub fn new_point_light(pos: Vec3<f32>, diffuse: Vec4<f32>, specular: Vec4<f32>,
	                       constant_attenuation: f32, linear_attenuation: f32, quadratic_attenuation: f32) -> Light {
		Light {
			pos: pos.to_pnt().to_homogeneous().to_vec(),
			diffuse: diffuse,
			specular: specular,
			constant_attenuation : constant_attenuation,
			linear_attenuation   : linear_attenuation,
			quadratic_attenuation: quadratic_attenuation,
			
			spot_cutoff: util::to_rad(180.0),
			spot_exponent: 0.0,
			spot_direction: Vec3::zero(),
		}
	}
	
	/// Constructs a new spotlight
	/// 
	/// `cutoff` is how wide the spotlight is, in radians.
	/// `exponent` is how 'focused' the spotlight is.
	pub fn new_spotlight(pos: Vec3<f32>, dir: Vec3<f32>, diffuse: Vec4<f32>, specular: Vec4<f32>,
	                     constant_attenuation: f32, linear_attenuation: f32, quadratic_attenuation: f32,
	                     cutoff: f32, exponent: f32) -> Light {
		Light {
			pos: pos.to_pnt().to_homogeneous().to_vec(),
			diffuse : diffuse,
			specular: specular,
			constant_attenuation : constant_attenuation,
			linear_attenuation   : linear_attenuation,
			quadratic_attenuation: quadratic_attenuation,
			spot_cutoff   : cutoff,
			spot_exponent : exponent,
			spot_direction: dir,
		}
	}
	
	/// Constructs a light that is off. (It has no output).
	pub fn off() -> Light {
		Light::new_directional(Vec3::zero(), Vec4::zero(), Vec4::zero())
	}
}

#[derive(Copy, Clone)]
pub struct Material {
	pub ambient: Vec4<f32>,
	pub diffuse: Vec4<f32>,
	pub specular: Vec4<f32>,
	pub shininess: f32,
}
impl Material {
	pub fn new(ambient: Vec4<f32>, diffuse: Vec4<f32>, specular: Vec4<f32>, shininess: f32) -> Material {
		Material {
			ambient: ambient,
			diffuse: diffuse,
			specular: specular,
			shininess: shininess,
		}
	}
	/// Returns a copy of the material, but with ambient reflection `r`.
	pub fn with_ambient(mut self, r: Vec4<f32>) -> Material {
		self.ambient = r;
		self
	}
	/// Returns a copy of the material, but with diffuse reflection `r`.
	pub fn with_diffuse(mut self, r: Vec4<f32>) -> Material {
		self.diffuse = r;
		self
	}
	/// Returns a copy of the material, but with specular reflection `r`.
	pub fn with_specular(mut self, r: Vec4<f32>) -> Material {
		self.specular = r;
		self
	}
	/// Returns a copy of the material, but with ambient, diffuse and specular reflection scaled by a color.
	pub fn with_scale_rgba(mut self, scale: Vec4<f32>) -> Material {
		self.ambient  = self.ambient  * scale;
		self.diffuse  = self.diffuse  * scale;
		self.specular = self.specular * scale;
		self
	}
}

/// RGB Color
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
	/// Constructs a new color with `r`, `g` and `b` being the same.
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
