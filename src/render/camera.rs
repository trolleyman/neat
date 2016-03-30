use std::f32;

use util::clamp;

use cgmath::*;

#[derive(Copy, Clone)]
pub struct Camera {
	pos: Vector3<f32>,
	yrot: f32,
	xrot: f32,
	view_mat: Matrix4<f32>,
}
impl Camera {
	pub fn new(pos: Vector3<f32>) -> Camera {
		let mut c = Camera {
			pos: pos,
			yrot: 0.0,
			xrot: 0.0,
			view_mat: Matrix4::identity(),
		};
		c.calculate_view_matrix();
		c
	}
	
	pub fn pos(&self) -> Vector3<f32> {
		self.pos
	}
	
	fn calculate_view_matrix(&mut self) {
		let pos = Matrix4::from_translation(-self.pos);
		let yrot: Matrix4<f32> = Quaternion::from_axis_angle(vec3(1.0, 0.0, 0.0), rad(-self.yrot)).into();
		let xrot: Matrix4<f32> = Quaternion::from_axis_angle(vec3(0.0, 1.0, 0.0), rad(-self.xrot)).into();
		self.view_mat = yrot * xrot * pos;
	}
	
	pub fn view_matrix(&self) -> Matrix4<f32> {
		self.view_mat
	}
	
	pub fn translate(&mut self, v: Vector3<f32>) {
		self.pos = self.pos + v;
		self.calculate_view_matrix();
	}
	
	// The mouse moved
	pub fn mouse_moved(&mut self, x: i32, y: i32) {
		let x = x as f32 * -0.008;
		let y = y as f32 * -0.008;
		self.look(x, y);
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
		
		self.calculate_view_matrix();
	}
}
