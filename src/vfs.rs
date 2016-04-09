//! Virtual File System
use std::io::prelude::*;
use std::io;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::process::exit;

use glium::*;
use glium::backend::Facade;
use rusttype::FontCollection;

// TODO: Make sure relative paths are in the right place relative to the executable.
// For now, just use CARGO_MANIFEST_DIR as the path.
const BASE_DIR: &'static str = env!("CARGO_MANIFEST_DIR");

fn assert_is_dir<P: AsRef<Path>>(dir: P) -> Result<(), String> {
	let dir = dir.as_ref();
	if !dir.is_dir() {
		Err(format!("Not a directory: '{}'", dir.display()))
	} else {
		Ok(())
	}
}

fn try_read_file_bytes<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, String> {
	fn get_contents(path: &Path) -> io::Result<Vec<u8>> {
		let mut f = File::open(path)?;
		let mut contents = Vec::with_capacity(f.metadata()?.len() as usize);
		f.read_to_end(&mut contents)?;
		Ok(contents)
	}
	
	let path = path.as_ref();
	get_contents(path).map_err(|e| {
		format!("Unreadable file '{}': {}", path.display(), e)
	})
}

fn try_read_file_string<P: AsRef<Path>>(path: P) -> Result<String, String> {
	fn get_contents(path: &Path) -> io::Result<String> {
		let mut f = File::open(path)?;
		let mut contents = String::with_capacity(f.metadata()?.len() as usize);
		f.read_to_string(&mut contents)?;
		Ok(contents)
	}
	
	let path = path.as_ref();
	get_contents(path).map_err(|e| {
		format!("Unreadable file '{}': {}", path.display(), e)
	})
}

pub fn load_shader<F: Facade>(facade: &F, name: &str) -> Program {
	match try_load_shader(facade, name) {
		Ok(program) => program,
		Err(e) => {
			error!("Cannot load shader '{}': {}", name, e);
			exit(1);
		}
	}
}

/// Loads a shader named `name`.
/// Looks for fragment shaders in `"shaders/" + name + ".frag"`
/// Looks for vertex shaders in `"shaders/" + name + ".vert"`
// TODO: Other shader types
pub fn try_load_shader<F: Facade>(facade: &F, name: &str) -> Result<Program, String> {
	let base_dir = PathBuf::from(BASE_DIR);
	
	let name = String::from(name);
	
	let shaders_dir = base_dir.join("shaders");
	assert_is_dir(&shaders_dir)?;
	
	let vert = try_read_file_string(shaders_dir.join(name.clone() + ".vert"))?;
	
	let frag = try_read_file_string(shaders_dir.join(name.clone() + ".frag"))?;
	
	match Program::from_source(facade, &vert, &frag, None) {
		Ok(p) => Ok(p),
		Err(e) => Err(format!("Shader '{}' could not be compiled:\n{}", name, e)),
	}
}

/// Loads a font from a file in the fonts/ folder.
/// Ensures that the font at `index` is valid.
pub fn load_font(name: &str, index: usize) -> FontCollection<'static> {
	match try_load_font(name, index) {
		Ok(font) => font,
		Err(e) => {
			error!("Cannot load font '{}': {}", name, e);
			exit(1);
		}
	}
}

/// Loads a font from a file in the fonts/ folder.
/// Ensures that the font at `index` is valid.
pub fn try_load_font(name: &str, index: usize) -> Result<FontCollection<'static>, String> {
	let base_dir = PathBuf::from(BASE_DIR);
	let fonts_dir = base_dir.join("fonts");
	let font_path = fonts_dir.join(name);
	let bytes = try_read_file_bytes(&font_path)?;
	
	let collection = FontCollection::from_bytes(bytes);
	match collection.font_at(index) {
		Some(_) => Ok(collection),
		None => Err(format!("Invalid font: '{}'", font_path.display()))
	}
}
