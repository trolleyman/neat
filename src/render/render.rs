use std::rc::Rc;
use std::io::{self, Read};
use std::fs::File;
use std::path::{Path, PathBuf};

use glium::backend::{Context, Facade};
use glium::backend::glutin_backend::{GlutinFacade, PollEventsIter};
use glium::*;

use glutin::WindowBuilder;

use render::Color;
use math::Vec3;

/// Render handler.
pub struct Render {
	win: GlutinFacade,
	_context: Rc<Context>,
	frame: Frame,
	
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
		
		win.get_window().unwrap().show();
		
		let ctx = win.get_context().clone();
		Render {
			win: win,
			_context: ctx,
			frame: frame,
			
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
		
		let shaders_dir = PathBuf::from("shaders");
		if !shaders_dir.is_dir() {
			return Err(format!("`shaders/` is not a directory."));
		}
		let shader_base = shaders_dir.join(name);
		
		let frag_path = shader_base.join(".frag");
		let frag = match get_source(&*frag_path) {
			Ok(s) => s,
			Err(e) => return Err(format!("Could not read shader file at `{}`: {}", frag_path.display(), e)),
		};
		
		let vert_path = shader_base.join(".vert");;
		let vert = match get_source(&*vert_path) {
			Ok(s) => s,
			Err(e) => return Err(format!("Could not read shader file at `{}`: {}", vert_path.display(), e)),
		};
		
		match Program::from_source(facade, &frag, &vert, None) {
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
	
	pub fn draw_sphere(&mut self, pos: Vec3, size: f32, color: Color) {
		
	}
}

impl Drop for Render {
	fn drop(&mut self) {
		self.frame.set_finish().ok();
	}
}
