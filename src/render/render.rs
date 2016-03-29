use std::rc::Rc;
use std::io::{self, Read};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::mem;

use glium::backend::{Context, Facade};
use glium::backend::glutin_backend::{GlutinFacade, PollEventsIter};
use glium::*;

use glutin::WindowBuilder;

use cgmath::{self, Vector3, Matrix4, SquareMatrix};

use render::{Camera, Color, Mesh};

/// Render handler.
pub struct Render {
	win: GlutinFacade,
	_context: Rc<Context>,
	frame: Frame,
	
	projection: Matrix4<f32>,	
	camera: Camera,
	
	sphere: Mesh,
	
	simple_shader: Program,
}
impl Render {
	pub fn new() -> Render {
		Render::with_size(800, 600)
	}

	pub fn with_size(w: u32, h: u32) -> Render {
		let win = match WindowBuilder::new()
			.with_dimensions(w, h)
			.with_title("NEAT".into())
			.with_visibility(false)
			.build_glium() {
			Ok(w)  => w,
			Err(e) => ::error(format!("Could not initialize window: {}", e))
		};
		
		let mut frame = win.draw();
		frame.clear_color(0.0, 0.0, 0.0, 0.0);
		frame.finish().ok();
		
		let frame = win.draw();
		
		let simple_shader = match Render::load_shader(&win, "simple") {
			Ok(i)  => i,
			Err(e) => ::error(e),
		};
		
		let sphere = Mesh::sphere(&win);
		
		win.get_window().unwrap().show();
		
		let ctx = win.get_context().clone();
		Render {
			win: win,
			_context: ctx,
			frame: frame,
			
			projection: cgmath::perspective(cgmath::deg(90.0), w as f32 / h as f32, 0.001, 1000.0),
			
			camera: Camera::new(),
			
			sphere: sphere,
			
			simple_shader: simple_shader,
		}
	}
	
	/// Loads a shader named `name`.
	/// Looks for fragment shaders in `"shaders/" + name + ".frag"`
	/// Looks for vertex shaders in `"shaders/" + name + ".vert"`
	/// TODO: Other shader types
	/// TODO: Some sort of cache
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

	pub fn poll_events<'a>(&'a self) -> PollEventsIter<'a> {
		self.win.poll_events()
	}

	pub fn frame(&mut self) -> &mut Frame {
		&mut self.frame
	}

	pub fn swap(&mut self) {
		self.frame.set_finish().ok();
		self.frame = self.win.draw();
	}
	
	pub fn draw_sphere(&mut self, pos: Vector3<f32>, size: f32, color: Color) {
		// Scale * Rotation * Translation
		let model = Matrix4::from_scale(size)
			* Matrix4::identity()
			* Matrix4::from_translation(pos);
				
		unsafe {
			self.frame.draw(
				self.sphere.vertices(),
				self.sphere.indices(),
				&self.simple_shader,
				&uniform!{
					projection: mem::transmute::<Matrix4<f32>, [[f32; 4]; 4]>(self.projection),
					view:       mem::transmute::<Matrix4<f32>, [[f32; 4]; 4]>(self.camera.view_matrix()),
					model:      mem::transmute::<Matrix4<f32>, [[f32; 4]; 4]>(model),
					in_color:   mem::transmute::<Vector3<f32>, [f32; 3]>(color.into()),
				},
				&Default::default()
			).unwrap();
		}
	}
}

impl Drop for Render {
	fn drop(&mut self) {
		self.frame.set_finish().ok();
	}
}
