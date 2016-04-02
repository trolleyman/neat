
mod color;
mod render;
mod camera;
mod mesh;
mod font;

pub use self::render::*;
pub use self::color::Color;
pub use self::camera::Camera;
pub use self::mesh::{SimpleVertex, Mesh};
pub use self::font::FontRender;

use std::io::prelude::*;
use std::io;
use std::path::{Path, PathBuf};
use std::fs::File;

use glium::*;
use glium::backend::Facade;
use rusttype::FontCollection;

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

/// Loads a font from a file.
/// Ensures that the font at `index` is valid.
fn load_font<P: AsRef<Path>>(path: P, index: usize) -> io::Result<FontCollection<'static>> {
	let path = path.as_ref();
	let mut file = File::open(path)?;
	
	let mut bytes = Vec::with_capacity(file.metadata()?.len() as usize);
	file.read_to_end(&mut bytes)?;
	
	let collection = FontCollection::from_bytes(bytes);
	match collection.font_at(index) {
		Some(_) => Ok(collection),
		None    => Err(io::Error::new(io::ErrorKind::InvalidData, format!("The font at '{}' cannot be read.", path.display()))),
	}
}
