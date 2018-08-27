use prelude::*;
use std::rc::Rc;
use std::cell::Ref;

use glium::*;
use glium::backend::Facade;
use glium::backend::glutin::Display;
use glium::uniforms::UniformsStorage;
use glutin::{ContextBuilder, EventsLoop, GlWindow, Robustness, WindowBuilder, Window};

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
	} else {
		fn os_focus_window(win: &Window) -> Result<(), ()> {
			// Don't do anything
			Ok(())
		}
	}
}

/// Tries to bring `win` into focus.
/// 
/// # Returns
/// Ok if the focus suceeded
/// Err if the focus failed
#[inline]
fn focus_window(win: &Window) -> Result<(), ()> {
	os_focus_window(win)
}

const SIMPLE_SHADER_NAME: &'static str = "simple";
const PHONG_SHADER_NAME: &'static str = "phong";

/// Render handler.
pub struct Render {
	/// Display backend
	display: Display,
	/// OpenGL handle
	ctx: Rc<Context>,
	/// Current framebuffer handle
	frame: Frame,
	
	/// Projection matrix
	projection: Matrix4<f32>,
	camera: Camera,
	
	ambient_light: Vector4<f32>,
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
	pub fn new(events_loop: &EventsLoop, camera: Camera, settings: &Settings) -> Result<Render, String> {
		// Setup window settings
		let win_builder = WindowBuilder::new()
			.with_dimensions((settings.w, settings.h).into())
			.with_title("NEAT")
			.with_visibility(false);
		
		// Setup OpenGL context settings
		let ctx_builder = ContextBuilder::new()
			.with_depth_buffer(8)
			.with_vsync(settings.vsync)
			.with_gl_robustness(Robustness::TryRobustLoseContextOnReset);
		
		// Build OpenGL window
		let gl_window = GlWindow::new(win_builder, ctx_builder, &events_loop)
			.map_err(|e| format!("Error building window: {}", e))?;
		
		// Build display
		let display = Display::from_gl_window(gl_window)
			.map_err(|e| format!("Error building OpenGL context: {}", e))?;
		
		// Build & clear framebuffer
		let mut frame = display.draw();
		Render::clear_frame(&mut frame);
		frame.finish().ok();
		let frame = display.draw();
		let ctx = display.get_context().clone();
		
		// Load shaders
		let simple_shader = vfs::load_shader(&ctx, SIMPLE_SHADER_NAME);
		let phong_shader = vfs::load_shader(&ctx, PHONG_SHADER_NAME);
		
		// Setup font renderer
		let font_render = FontRender::new(ctx.clone());
		
		let mut r = Render {
			display,
			ctx,
			frame,
			
			projection: Matrix4::one(),
			camera,
			
			ambient_light: Vector4::zero(),
			light: Light::off(),
			wireframe_mode: false,
			simple_shader: simple_shader,
			phong_shader: phong_shader,
			font_render: font_render,
		};
		r.resize();
		Ok(r)
	}
	
	/// Clears the color and depth buffers of `frame`
	fn clear_frame(frame: &mut Frame) {
		frame.clear_color(0.0, 0.0, 0.0, 0.0);
		frame.clear_depth(1.0);
	}
	
	pub fn set_ambient_light(&mut self, ambient_light: Vector4<f32>) {
		self.ambient_light = ambient_light;
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
	
	/// Shows the window to the user
	pub fn show(&mut self) {
		self.window().show();
	}
	
	/// Tries to reload the shaders currently used.
	/// 
	/// If there was an error compiling the shaders, the current shaders are not affected and
	/// an error message is returned.
	pub fn reload_shaders(&mut self) -> Result<(), String> {
		let simple = vfs::try_load_shader(&self.ctx, SIMPLE_SHADER_NAME)?;
		let phong  = vfs::try_load_shader(&self.ctx, PHONG_SHADER_NAME)?;
		
		self.simple_shader = simple;
		self.phong_shader = phong;
		Ok(())
	}
	
	/// Draws the `s` on the screen at [`x`, `y`] with pt size `scale` in white.
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
		self.projection = Perspective3::new(w as f32 / h as f32, util::to_rad(90.0), 0.001, 1000.0).to_homogeneous();
	}
	
	/// Tries to grab the focus of the window
	pub fn try_focus(&mut self) -> Result<(), ()> {
		focus_window(&self.window())
	}
	
	/// Grabs the cursor.
	pub fn input_grab(&mut self) {
		self.window().grab_cursor(true).ok();
		self.window().hide_cursor(true);
	}
	
	/// Lets the cursor go.
	pub fn input_normal(&mut self) {
		self.window().grab_cursor(false).ok();
		self.window().hide_cursor(false);
	}
	
	pub fn window(&self) -> Ref<GlWindow> {
		self.display.gl_window()
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
		self.frame = self.display.draw();
		Render::clear_frame(&mut self.frame);
	}
	
	/// Executes all opengl commands in the queue. Use only for debugging purposes.
	pub fn flush(&mut self) {
		self.ctx.finish();
	}
	
	/// Render a simple list of vertices in a specified color.
	pub fn render_simple(&mut self, vs: &VertexBuffer<SimpleVertex>, is: &IndexBuffer<u16>, model: Matrix4<f32>, col: Color) {
		let mvp = self.projection * self.camera.view_matrix() * model;
		
		self.frame.draw(
			vs,
			is,
			&self.simple_shader,
			&uniform! {
				mvp  : *mvp.as_ref(),
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
	pub fn render_lit(&mut self, vs: &VertexBuffer<LitVertex>, is: &IndexBuffer<u16>, model: Matrix4<f32>, texture: &Texture2d, material: &Material) {
		let m = model;
		let v = self.camera.view_matrix();
		let p = self.projection;
		let mvp = p * v * m;
		let v_inv = self.camera.view_matrix().try_inverse().unwrap_or(Matrix4::one());
		let normal_mat = m.try_inverse().unwrap_or(Matrix4::one()).transpose();
		
		let uniforms = UniformsStorage::new("mvp", *mvp.as_ref());
		let uniforms = uniforms.add("model"     , *m.as_ref());
		let uniforms = uniforms.add("v_inv"     , *v_inv.as_ref());
		let uniforms = uniforms.add("normal_mat", *util::mat4_upper_left(normal_mat).as_ref());
		let uniforms = uniforms.add("tex", texture);
		let uniforms = uniforms.add("ambient", *self.ambient_light.as_ref());
		/*
		let light_buf = UniformBuffer::immutable(&self.ctx, [light]);
		let material_buf = UniformBuffer::immutable(&self.ctx, [material]);
		
		let uniforms = uniforms.add("light", light_buf);
		let uniforms = uniforms.add("material", material_buf);*/
		let uniforms = uniforms.add("light_pos", *self.light.pos.as_ref());
		let uniforms = uniforms.add("light_diffuse" , *self.light.diffuse.as_ref());
		let uniforms = uniforms.add("light_specular", *self.light.specular.as_ref());
		let uniforms = uniforms.add("light_constant_attenuation" , self.light.constant_attenuation);
		let uniforms = uniforms.add("light_linear_attenuation"   , self.light.linear_attenuation);
		let uniforms = uniforms.add("light_quadratic_attenuation", self.light.quadratic_attenuation);
		let uniforms = uniforms.add("light_spot_cutoff"   , self.light.spot_cutoff);
		let uniforms = uniforms.add("light_spot_exponent" , self.light.spot_exponent);
		let uniforms = uniforms.add("light_spot_direction", *self.light.spot_direction.as_ref());
		
		let uniforms = uniforms.add("material_ambient"  , *material.ambient.as_ref());
		let uniforms = uniforms.add("material_diffuse"  , *material.diffuse.as_ref());
		let uniforms = uniforms.add("material_specular" , *material.specular.as_ref());
		let uniforms = uniforms.add("material_shininess", material.shininess);
		
		self.frame.draw(
			vs,
			is,
			&self.phong_shader,
			&uniforms,
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
		).map_err(|e| error!("Draw failed: {}", e)).ok();
	}
}

impl Drop for Render {
	fn drop(&mut self) {
		// Probably don't need to do this, but just in case.
		self.frame.set_finish().ok();
	}
}
