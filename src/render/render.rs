use std::rc::Rc;
use std::mem;
use std::process::exit;

use glium::backend::{Context, Facade};
use glium::backend::glutin_backend::{GlutinFacade, PollEventsIter, WinRef};
use glium::*;
use glutin::{CursorState, WindowBuilder, Window};
use na::{Mat4, Persp3, Eye};

use util;
use vfs::load_shader;
use render::{FontRender, Camera, Color, SimpleVertex};

cfg_if! {
	if #[cfg(target_os = "windows")] {
		fn focus_window(win: &Window) -> Result<(), ()> {
			use glutin::os::windows::WindowExt;
			use user32;
			unsafe {
				let hwnd = win.get_hwnd() as *mut _;
				// FIXME: God damn focus keeps on failing. It grabs input, but doesn't get to the top of the z-order.
				let fail = user32::SetForegroundWindow(hwnd) == 0;
				if fail {
					warn!("Focus failed");
					Err(())
				} else {
					Ok(())
				}
			}
		}
	} else if #[cfg(target_os = "macos")] {
		fn focus_window(win: &Window) -> Result<(), ()> {
			// TODO
			false
		}
	} else if #[cfg(target_os = "linux")] {
		fn focus_window(win: &Window) -> Result<(), ()> {
			// TODO
			false
		}
	} else {
		fn focus_window(win: &Window) -> Result<(), ()> {
			// Don't do anything
			false
		}
	}
}

/// Render handler.
pub struct Render {
	win: GlutinFacade,
	ctx: Rc<Context>,
	frame: Frame,
	
	projection: Mat4<f32>,
	camera: Camera,
	
	simple_shader: Program,
	font_render: FontRender,
}
impl Render {
	pub fn new(camera: Camera) -> Render {
		Render::with_size(camera, 800, 600)
	}
	
	fn clear_frame(frame: &mut Frame) {
		frame.clear_color(0.0, 0.0, 0.0, 0.0);
		frame.clear_depth(1.0);
	}

	pub fn with_size(camera: Camera, w: u32, h: u32) -> Render {
		let win = match WindowBuilder::new()
			.with_dimensions(w, h)
			.with_title("NEAT".into())
			.with_visibility(false)
			.with_depth_buffer(24)
			.with_vsync()
			.build_glium() {
				Ok(w)  => w,
				Err(e) => {
					error!("Could not initialize window: {}", e);
					exit(1);
				}
		};
		
		let mut frame = win.draw();
		Render::clear_frame(&mut frame);
		frame.finish().ok();
		let frame = win.draw();
		
		let simple_shader = match load_shader(&win, "simple") {
			Ok(i)  => i,
			Err(e) => {
				error!("{}", e);
				exit(1);
			},
		};
		
		let ctx = win.get_context().clone();
		
		let font_render = FontRender::new(ctx.clone());
		
		let mut r = Render {
			win: win,
			ctx: ctx,
			frame: frame,
			
			projection: Mat4::new_identity(4),
			camera: camera,
			
			simple_shader: simple_shader,
			font_render: font_render,
		};
		r.resize();
		r.win.get_window().map(|w| w.show());
		r
	}
	
	pub fn draw_str(&mut self, s: &str, x: f32, y: f32, scale: f32) {
		self.draw_str_color(s, x, y, scale, Color::WHITE);
	}
	pub fn draw_str_color(&mut self, s: &str, x: f32, y: f32, scale: f32, color: Color) {
		let (screen_w, screen_h) = self.frame.get_dimensions();
		self.font_render.draw_str(&mut self.frame, s, x, y, screen_w as f32, screen_h as f32, scale, color);
	}
	
	pub fn camera(&self) -> &Camera {
		&self.camera
	}
	
	pub fn set_camera(&mut self, cam: Camera) {
		self.camera = cam;
	}
	
	/// Resizes the renderer
	pub fn resize(&mut self) {
		let (w, h) = self.frame.get_dimensions();
		self.projection = Persp3::new(w as f32 / h as f32, util::to_rad(90.0), 0.001, 1000.0).to_mat();
	}
	
	pub fn get_window(&self) -> Option<WinRef> {
		self.win.get_window()
	}
	
	pub fn poll_events<'a>(&'a self) -> PollEventsIter<'a> {
		self.win.poll_events()
	}
	
	pub fn focus(&mut self) -> Result<(), ()> {
		if let Some(win) = self.win.get_window() {
			if focus_window(&win).is_ok() {
				win.set_cursor_state(CursorState::Grab).ok();
				Ok(())
			} else {
				win.set_cursor_state(CursorState::Normal).ok();
				Err(())
			}
		} else {
			Err(())
		}
	}
	
	pub fn unfocus(&mut self) {
		info!("Window unfocused");
		self.win.get_window().map(|w| w.set_cursor_state(CursorState::Normal));
	}
	
	pub fn context(&mut self) -> &Rc<Context> {
		&self.ctx
	}
	
	pub fn frame(&mut self) -> &mut Frame {
		&mut self.frame
	}

	pub fn swap(&mut self) {
		trace!("Swapping buffers...");
		self.frame.set_finish().ok();
		self.frame = self.win.draw();
		Render::clear_frame(&mut self.frame);
	}
	
	pub fn render_simple(&mut self, vs: &VertexBuffer<SimpleVertex>, is: &IndexBuffer<u32>, model: Mat4<f32>, col: Color) {
		self.frame.draw(
			vs,
			is,
			&self.simple_shader,
			&uniform! {
				projection: unsafe { mem::transmute::<Mat4<f32>, [[f32; 4]; 4]>(self.projection) },
				view:       unsafe { mem::transmute::<Mat4<f32>, [[f32; 4]; 4]>(self.camera.view_matrix()) },
				model:      unsafe { mem::transmute::<Mat4<f32>, [[f32; 4]; 4]>(model) },
				color:      unsafe { mem::transmute::<Color, [f32; 3]>(col) },
			},
			&DrawParameters {
				depth: Depth {
					test: DepthTest::IfLess,
					write: true,
					..Default::default()
				},
				backface_culling: BackfaceCullingMode::CullClockwise,
				..Default::default()
			}
		).map_err(|e| error!("Draw failed: {:?}", e)).ok();
	}
}

impl Drop for Render {
	fn drop(&mut self) {
		self.frame.set_finish().ok();
	}
}
