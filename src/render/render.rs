use prelude::*;
use std::rc::Rc;
use std::mem;
use std::process::exit;

use glium::backend::Facade;
use glium::backend::glutin_backend::{GlutinFacade, PollEventsIter, WinRef};
use glium::*;
use glutin::{CursorState, WindowBuilder, Window};

use util;
use vfs;
use settings::Settings;
use render::{FontRender, Camera, Color, SimpleVertex, LitVertex, Light, Material};

cfg_if! {
	if #[cfg(target_os = "windows")] {
		fn os_focus_window(win: &Window) -> Result<(), ()> {
			use glutin::os::windows::WindowExt;
			use user32;
			unsafe {
				let hwnd = win.get_hwnd() as *mut _;
				let fail = user32::SetForegroundWindow(hwnd) == 0;
				if fail {
					warn!("Focus failed");
					Err(())
				} else {
					user32::SetActiveWindow(hwnd);
					Ok(())
				}
			}
		}
	} else if #[cfg(target_os = "macos")] {
		fn os_focus_window(win: &Window) -> Result<(), ()> {
			// TODO
			Err(())
		}
	} else if #[cfg(target_os = "linux")] {
		fn os_focus_window(win: &Window) -> Result<(), ()> {
			// TODO
			Err(())
		}
	} else {
		fn os_focus_window(win: &Window) -> Result<(), ()> {
			// Don't do anything
			Err(())
		}
	}
}

/// Tries to bring `win` into focus.
/// 
/// # Returns
/// Ok if the focus suceeded
/// Err if the focus failed
fn focus_window(win: &Window) -> Result<(), ()> {
	os_focus_window(win)
}

/// Render handler.
pub struct Render {
	/// Window handle
	win: GlutinFacade,
	/// OpenGL handle
	ctx: Rc<Context>,
	/// Current framebuffer handle
	frame: Frame,
	
	/// Projection matrix
	projection: Mat4<f32>,
	camera: Camera,
	
	light: Light,
	wireframe_mode: bool,
	simple_shader: Program,
	phong_shader: Program,
	font_render: FontRender,
}
impl Render {
	/// Constructs a new `Render` object.
	/// 
	/// In doing so it opens a window, loads the necessary shaders and initializes the font renderer.
	pub fn new(camera: Camera, settings: &Settings) -> Render {
		let win = {
			let mut builder = WindowBuilder::new()
				.with_dimensions(settings.w, settings.h)
				.with_title("NEAT".into())
				.with_visibility(false)
				.with_depth_buffer(24);
			
			if settings.vsync {
				builder = builder.with_vsync();
			}
			
			match builder.build_glium() {
				Ok(w)  => w,
				Err(e) => {
					error!("Could not initialize window: {}", e);
					exit(1);
				}
			}
		};
		
		let mut frame = win.draw();
		Render::clear_frame(&mut frame);
		frame.finish().ok();
		let frame = win.draw();
		let ctx = win.get_context().clone();
		
		let simple_shader = vfs::load_shader(&ctx, "simple");
		
		let phong_shader = vfs::load_shader(&ctx, "phong");
		
		let font_render = FontRender::new(ctx.clone());
		
		let mut r = Render {
			win: win,
			ctx: ctx,
			frame: frame,
			
			projection: Mat4::one(),
			camera: camera,
			
			light: Light::off(),
			wireframe_mode: false,
			simple_shader: simple_shader,
			phong_shader: phong_shader,
			font_render: font_render,
		};
		r.resize();
		r.win.get_window().map(|w| w.show());
		r
	}
	
	/// Clears the color and depth buffers of `frame`
	fn clear_frame(frame: &mut Frame) {
		frame.clear_color(0.0, 0.0, 0.0, 0.0);
		frame.clear_depth(1.0);
	}
	
	pub fn set_light(&mut self, light: Light) {
		self.light = light;
	}
	
	pub fn set_wireframe_mode(&mut self, mode: bool) {
		self.wireframe_mode = mode;
	}
	
	pub fn camera(&self) -> &Camera {
		&self.camera
	}
	
	pub fn set_camera(&mut self, cam: Camera) {
		self.camera = cam;
	}
	
	/// Draws the `s` on the screen at [`x`, `y`] with pt size `scale`.
	pub fn draw_str(&mut self, s: &str, x: f32, y: f32, scale: f32) {
		self.draw_str_color(s, x, y, scale, Color::WHITE);
	}
	/// Draws the `s` on the screen at [`x`, `y`] with pt size `scale` in `color`.
	pub fn draw_str_color(&mut self, s: &str, x: f32, y: f32, scale: f32, color: Color) {
		let (screen_w, screen_h) = self.frame.get_dimensions();
		self.font_render.draw_str(&mut self.frame, s, x, y, screen_w as f32, screen_h as f32, scale, color);
	}
	
	/// Resizes the renderer to the current framebuffer's dimensions.
	pub fn resize(&mut self) {
		let (w, h) = self.frame.get_dimensions();
		self.projection = Persp3::new(w as f32 / h as f32, util::to_rad(90.0), 0.001, 1000.0).to_mat();
	}
	
	pub fn get_window(&self) -> Option<WinRef> {
		self.win.get_window()
	}
	
	/// Gets an iterator that polls the events of the current window
	pub fn poll_events<'a>(&'a self) -> PollEventsIter<'a> {
		self.win.poll_events()
	}
	
	/// Tries to grab the focus of the window. If it does it also sets the cursor grabbing state.
	pub fn try_focus(&mut self) -> Result<(), ()> {
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
	
	/// Grabs the cursor.
	pub fn input_grab(&self) {
		self.get_window().map(|w| w.set_cursor_state(CursorState::Grab));
	}
	
	/// Lets the cursor go.
	pub fn input_normal(&self) {
		self.get_window().map(|w| w.set_cursor_state(CursorState::Normal));
	}
	
	pub fn context(&mut self) -> &Rc<Context> {
		&self.ctx
	}
	
	pub fn frame(&mut self) -> &mut Frame {
		&mut self.frame
	}
	
	/// Flush the current output of OpenGL to the scrren.
	/// 
	/// Swaps the framebuffers, if the window is double buffered.
	pub fn swap(&mut self) {
		trace!("Swapping buffers...");
		self.frame.set_finish().ok();
		self.frame = self.win.draw();
		Render::clear_frame(&mut self.frame);
	}
	
	/// Executes all opengl commands in the queue. Use only for debugging purposes.
	pub fn flush(&mut self) {
		self.ctx.finish();
	}
	
	/// Render a simple list of vertices in a specified color.
	pub fn render_simple(&mut self, vs: &VertexBuffer<SimpleVertex>, is: &IndexBuffer<u16>, model: Mat4<f32>, col: Color) {
		let mvp = self.projection * self.camera.view_matrix() * model;
		
		self.frame.draw(
			vs,
			is,
			&self.simple_shader,
			&uniform! {
				mvp  : unsafe { mem::transmute::<Mat4<f32>, [[f32; 4]; 4]>(mvp) },
				color: col.into_array(),
			},
			&DrawParameters {
				depth: Depth {
					test: DepthTest::IfLess,
					write: true,
					..Default::default()
				},
				polygon_mode: if self.wireframe_mode { PolygonMode::Line } else { PolygonMode::Fill },
				backface_culling: BackfaceCullingMode::CullClockwise,
				..Default::default()
			}
		).map_err(|e| error!("Draw failed: {:?}", e)).ok();
	}
	
	/// Render a lit, textured surface.
	pub fn render_lit(&mut self, vs: &VertexBuffer<LitVertex>, is: &IndexBuffer<u16>, model: Mat4<f32>, texture: &Texture2d, material: &Material) {
		let mv = self.camera.view_matrix() * model;
		let mvp = self.projection * mv;
		let normal_mat = mv.inv().unwrap_or(Mat4::one()).transpose();
		
		self.frame.draw(
			vs,
			is,
			&self.phong_shader,
			&uniform! {
				mvp       : unsafe { mem::transmute::<Mat4<f32>, [[f32; 4]; 4]>(mvp) },
				model     : unsafe { mem::transmute::<Mat4<f32>, [[f32; 4]; 4]>(model) },
				normal_mat: unsafe { mem::transmute::<Mat4<f32>, [[f32; 4]; 4]>(normal_mat) },
				tex       : texture,
				iA: unsafe { mem::transmute::<Vec4<f32>, [f32; 4]>(self.light.intensity_ambient) },
				iS: unsafe { mem::transmute::<Vec4<f32>, [f32; 4]>(self.light.intensity_specular) },
				iD: unsafe { mem::transmute::<Vec4<f32>, [f32; 4]>(self.light.intensity_diffuse) },
				kA: unsafe { mem::transmute::<Vec4<f32>, [f32; 4]>(material.reflection_ambient) },
				kS: unsafe { mem::transmute::<Vec4<f32>, [f32; 4]>(material.reflection_specular) },
				kD: unsafe { mem::transmute::<Vec4<f32>, [f32; 4]>(material.reflection_diffuse) },
				shininess : material.shininess,
				light_pos : unsafe { mem::transmute::<Vec3<f32>, [f32; 3]>(self.light.pos) },
				camera_pos: unsafe { mem::transmute::<Vec3<f32>, [f32; 3]>(self.camera.pos()) },
			},
			&DrawParameters {
				depth: Depth {
					test: DepthTest::IfLess,
					write: true,
					..Default::default()
				},
				polygon_mode: if self.wireframe_mode { PolygonMode::Line } else { PolygonMode::Fill },
				backface_culling: BackfaceCullingMode::CullClockwise,
				..Default::default()
			}
		).map_err(|e| error!("Draw failed: {:?}", e)).ok();
	}
}

impl Drop for Render {
	fn drop(&mut self) {
		// Probably don't need to do this, but just in case.
		self.frame.set_finish().ok();
	}
}
