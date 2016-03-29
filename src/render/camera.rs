use std::f32;

use util::clamp;

use cgmath::{vec3, Vector3, Matrix4, SquareMatrix};

pub struct Camera {
	pos: Vector3<f32>,
	yrot: f32,
	xrot: f32,
	view_mat: Matrix4<f32>,
}
impl Camera {
	pub fn new() -> Camera {
		let mut c = Camera {
			pos: vec3(0.0, 0.0, 0.0),
			yrot: 0.0,
			xrot: 0.0,
			view_mat: Matrix4::identity(),
		};
		c.calculate_view_matrix();
		c
	}
	
	fn calculate_view_matrix(&mut self) {
		self.view_mat = Matrix4::identity();
		
	}
	
	pub fn view_matrix(&self) -> Matrix4<f32> {
		self.view_mat
	}
	
	pub fn translate(&mut self, v: Vector3<f32>) {
		self.pos = self.pos + v;
	}
	
	// Have a look around (radians)
	pub fn look(&mut self, x: f32, y: f32) {
		self.xrot += x;
		self.yrot += y;
		
		self.xrot %= f32::consts::PI * 2.;
		if self.xrot < 0.0 {
			self.xrot += f32::consts::PI * 2.;
		}
		
		self.yrot = clamp(self.yrot, f32::consts::PI / -2., f32::consts::PI / 2.);
	}
}
