use std::rc::Rc;
use std::io::{self, Read};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::mem;

use glium::backend::{Context, Facade};
use glium::backend::glutin_backend::{GlutinFacade, PollEventsIter};
use glium::*;

use glutin::{CursorState, WindowBuilder};

use cgmath::{self, Matrix4, SquareMatrix};

use render::{Camera, Color, SimpleVertex};

/// Render handler.
pub struct Render {
	win: GlutinFacade,
	_context: Rc<Context>,
	frame: Frame,
	
	projection: Matrix4<f32>,
	
	camera: Camera,
	
	simple_shader: Program,
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
			.build_glium() {
			Ok(w)  => w,
			Err(e) => ::error(format!("Could not initialize window: {}", e))
		};
		
		let mut frame = win.draw();
		Render::clear_frame(&mut frame);
		frame.finish().ok();
		let frame = win.draw();
		
		let simple_shader = match Render::load_shader(&win, "simple") {
			Ok(i)  => i,
			Err(e) => ::error(e),
		};
		
		let ctx = win.get_context().clone();
		let mut r = Render {
			win: win,
			_context: ctx,
			frame: frame,
			
			projection: Matrix4::identity(),
			
			camera: camera,
			
			simple_shader: simple_shader,
		};
		r.resize();
		r.win.get_window().map(|w| w.show());
		r
	}
	
	pub fn camera(&self) -> &Camera {
		&self.camera
	}
	
	pub fn set_camera(&mut self, cam: Camera) {
		self.camera = cam;
	}
	
	/// Loads a shader named `name`.
	/// Looks for fragment shaders in `"shaders/" + name + ".frag"`
	/// Looks for vertex shaders in `"shaders/" + name + ".vert"`
	/// TODO: Other shader types
	fn load_shader<F: Facade>(facade: &F, name: &str) -> Result<Program, String> {
		fn get_source(path: &Path) -> io::Result<String> {
			let mut f = File::open(path)?;
			let mut src = String::with_capacity(f.metadata()?.len() as usize);
			f.read_to_string(&mut src)?;
			Ok(src)
		}
		
		let name = String::from(name);
		
		let shaders_dir = PathBuf::from("shaders");
		if !shaders_dir.is_dir() {
			return Err(format!("`shaders/` is not a directory."));
		}
		
		let vert_path = shaders_dir.join(name.clone() + ".vert");
		let vert = match get_source(&*vert_path) {
			Ok(s) => s,
			Err(e) => return Err(format!("Could not read shader file at `{}`: {}", vert_path.display(), e)),
		};
		
		let frag_path = shaders_dir.join(name.clone() + ".frag");
		let frag = match get_source(&*frag_path) {
			Ok(s) => s,
			Err(e) => return Err(format!("Could not read shader file at `{}`: {}", frag_path.display(), e)),
		};
		
		match Program::from_source(facade, &vert, &frag, None) {
			Ok(p) => Ok(p),
			Err(e) => Err(format!("Could not compile shader `{}`: {}", name, e)),
		}
	}
	
	/// Resizes the renderer
	pub fn resize(&mut self) {
		if let Some((w, h)) = self.win.get_window().and_then(|w| w.get_inner_size_pixels()) {
			self.projection = cgmath::perspective(cgmath::deg(90.0), w as f32 / h as f32, 0.001, 1000.0);
		}
	}

	pub fn poll_events<'a>(&'a self) -> PollEventsIter<'a> {
		self.win.poll_events()
	}
	
	pub fn focus(&mut self) {
		self.win.get_window().map(|w| w.set_cursor_state(CursorState::Grab));
	}
	
	pub fn unfocus(&mut self) {
		self.win.get_window().map(|w| w.set_cursor_state(CursorState::Normal));
	}
	
	pub fn facade(&self) -> &GlutinFacade {
		&self.win
	}

	pub fn frame(&mut self) -> &mut Frame {
		&mut self.frame
	}

	pub fn swap(&mut self) {
		self.frame.set_finish().ok();
		self.frame = self.win.draw();
		Render::clear_frame(&mut self.frame);
	}
	
	pub fn render_simple(&mut self, vs: &VertexBuffer<SimpleVertex>, is: &IndexBuffer<u32>, model: Matrix4<f32>, col: Color) {
		self.frame.draw(
			vs,
			is,
			&self.simple_shader,
			&uniform! {
				projection: unsafe { mem::transmute::<Matrix4<f32>, [[f32; 4]; 4]>(self.projection) },
				view:       unsafe { mem::transmute::<Matrix4<f32>, [[f32; 4]; 4]>(self.camera.view_matrix()) },
				model:      unsafe { mem::transmute::<Matrix4<f32>, [[f32; 4]; 4]>(model) },
				in_color:   unsafe { mem::transmute::<Color, [f32; 3]>(col) },
			},
			&DrawParameters {
				depth: Depth {
					test: DepthTest::IfLess,
					write: true,
					..Default::default()
				},
				//backface_culling: BackfaceCullingMode::CullClockwise,
				..Default::default()
			}
		).unwrap();
	}
}

impl Drop for Render {
	fn drop(&mut self) {
		self.frame.set_finish().ok();
	}
}
