use std::f32;

use na::{Vec3, Mat4, UnitQuat, Eye, ToHomogeneous, Rot3};

use util::{self, clamp};

#[derive(Copy, Clone, Debug)]
pub struct Camera {
	pos: Vec3<f32>,
	yrot: f32,
	xrot: f32,
	view_mat: Mat4<f32>,
}
impl Camera {
	pub fn new(pos: Vec3<f32>) -> Camera {
		let mut c = Camera {
			pos: pos,
			yrot: 0.0,
			xrot: 0.0,
			view_mat: Mat4::new_identity(4),
		};
		c.calculate_view_matrix();
		c
	}
	
	pub fn pos(&self) -> Vec3<f32> {
		self.pos
	}
	
	fn calculate_view_matrix(&mut self) {
		let pos = util::mat4_translation(-self.pos);
		let rot_y = Rot3::new_with_euler_angles(-self.yrot, 0.0, 0.0).to_homogeneous();
		let rot_x = Rot3::new_with_euler_angles(0.0, -self.xrot, 0.0).to_homogeneous();
		self.view_mat = rot_y * rot_x * pos;
	}
	
	pub fn view_matrix(&self) -> Mat4<f32> {
		self.view_mat
	}
	
	pub fn translate(&mut self, v: Vec3<f32>) {
		let rot = UnitQuat::new(Vec3::new(0.0, self.xrot, 0.0));
		self.pos = self.pos + rot * v;
		self.calculate_view_matrix();
	}
	
	// The mouse moved
	pub fn mouse_moved(&mut self, screen_x: i32, screen_y: i32) {
		let x = screen_x as f32 * -0.008;
		let y = screen_y as f32 * -0.008;
		if screen_x != 0 && screen_y != 0 {
			debug!("mouse moved: {:3},{:3} look change: {},{}", x, y, -screen_x, -screen_y);
		}
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
