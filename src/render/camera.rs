use prelude::*;
use std::f32;

use na;

use util;

/// Structure holding the position and rotation of a camera
#[derive(Copy, Clone, Debug)]
pub struct Camera {
	pos: Vec3<f32>,
	yrot: f32,
	xrot: f32,
	view_mat: Option<Mat4<f32>>,
}
impl Camera {
	/// Constructs a new camera at the specified path.
	pub fn new(pos: Vec3<f32>) -> Camera {
		Camera {
			pos: pos,
			yrot: 0.0,
			xrot: 0.0,
			view_mat: None,
		}
	}
	
	pub fn pos(&self) -> Vec3<f32> {
		self.pos
	}
	
	/// Get the view matrix of the camera.
	pub fn view_matrix(&mut self) -> Mat4<f32> {
		let mat = if let Some(view_mat) = self.view_mat {
			view_mat
		} else {
			let pos = util::mat4_translation(-self.pos);
			let rot_y = Rot3::new_with_euler_angles(-self.yrot, 0.0, 0.0).to_homogeneous();
			let rot_x = Rot3::new_with_euler_angles(0.0, -self.xrot, 0.0).to_homogeneous();
			rot_y * rot_x * pos
		};
		self.view_mat = Some(mat);
		mat
	}
	
	/// Translate the camera by a specified amount, taking into account the rotation.
	pub fn translate(&mut self, v: Vec3<f32>) {
		let rot = UnitQuat::new(Vec3::new(0.0, self.xrot, 0.0));
		self.pos = self.pos + rot * v;
		self.view_mat = None;
	}
	
	/// Handle a mouse move on the screen by rotating the camera.
	pub fn mouse_moved(&mut self, screen_x: i32, screen_y: i32) {
		let x = screen_x as f32 * -0.008;
		let y = screen_y as f32 * -0.008;
		if screen_x != 0 && screen_y != 0 {
			trace!("mouse moved: {:3},{:3} look change: {},{}", x, y, -screen_x, -screen_y);
		}
		self.look(x, y);
	}
	
	/// Apply a rotation in the x and y direction (in radians)
	pub fn look(&mut self, x: f32, y: f32) {
		self.xrot += x;
		self.yrot += y;
		
		self.xrot %= f32::consts::PI * 2.;
		if self.xrot < 0.0 {
			self.xrot += f32::consts::PI * 2.;
		}
		
		self.yrot = na::clamp(self.yrot, f32::consts::PI / -2., f32::consts::PI / 2.);
		
		self.view_mat = None;
	}
}
