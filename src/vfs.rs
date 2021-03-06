//! Virtual File System.
//!
//! Handles the loading of shaders, textures and fonts.
use prelude::*;
use std::io::prelude::*;
use std::io;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::process::exit;
use std::rc::Rc;

use glium::*;
use glium::texture::RawImage2d;
use rusttype::{Font, FontCollection};
use image::{self, DynamicImage, ConvertBuffer};

/// Gets the base directory for all of the vfs operations.
fn try_get_base_dir() -> Result<PathBuf, String> {
	let mut path = ::std::env::current_exe()
		.map_err(|e| format!("unable to locate current executable: {}", e))?;
	
	path.pop();
	path.push("assets");
	assert_is_dir(&path)?;
	Ok(path)
}

/// Returns Err if the `path` is not a directory with a custom error message.
fn assert_is_dir<P: AsRef<Path>>(path: P) -> Result<(), String> {
	let path = path.as_ref();
	if !path.exists() {
		Err(format!("directory does not exist: '{}'", path.display()))
	} else if !path.is_dir() {
		Err(format!("not a directory: '{}'", path.display()))
	} else {
		Ok(())
	}
}

/// Trys to read the file at `path`.
///
/// Returns a custom error message on failure.
fn try_read_file_bytes<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, String> {
	fn get_contents(path: &Path) -> io::Result<Vec<u8>> {
		let mut f = File::open(path)?;
		let mut contents = Vec::with_capacity(f.metadata()?.len() as usize + 1);
		f.read_to_end(&mut contents)?;
		Ok(contents)
	}
	
	let path = path.as_ref();
	if !path.exists() {
		return Err(format!("file does not exist: '{}'", path.display()));
	} else if !path.is_file() {
		return Err(format!("not a file: '{}'", path.display()));
	}
	get_contents(path).map_err(|e| {
		format!("unreadable file '{}': {}", path.display(), e)
	})
}

/// Trys to read the file at `path`, converting it to a string.
///
/// Returns a custom error message on failure.
fn try_read_file_string<P: AsRef<Path>>(path: P) -> Result<String, String> {
	let path = path.as_ref();
	let bytes = try_read_file_bytes(path)?;
	String::from_utf8(bytes)
		.map_err(|e| format!("unreadable file '{}': {}", path.display(), e))
}

/// Tries to load an arbitrary data file from the `assets/` folder as bytes.
pub fn try_load_data_bytes<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, String> {
	fn inner_try(path: &Path) -> Result<Vec<u8>, String> {
		let base_dir = try_get_base_dir()?;
		try_read_file_bytes(base_dir.join(path))
	}
	let path = path.as_ref();
	inner_try(path)
		.map_err(|e| format!("could not load data file '{}': {}", path.display(), e))
}

/// Tries to load an arbitrary data file from the `assets/` folder as a string.
pub fn try_load_data_string<P: AsRef<Path>>(path: P) -> Result<String, String> {
	fn inner_try(path: &Path) -> Result<String, String> {
		let base_dir = try_get_base_dir()?;
		try_read_file_string(base_dir.join(path))
	}
	let path = path.as_ref();
	inner_try(path)
		.map_err(|e| format!("could not load data file '{}': {}", path.display(), e))
}

/// Loads the shader `name` from the `shaders/` folder.
/// 
/// If it finds a file with the name of the shader and the extension
/// - `.vert` it will load it as a vertex shader.
/// - `.frag` it will load it as a fragment shader.
/// - TODO: More shader types
/// 
/// Exits if
/// - the vertex shader could not be found/compiled.
/// - the fragment shader could not be found/compiled.
pub fn load_shader(ctx: &Rc<Context>, name: &str) -> Program {
	match try_load_shader(ctx, name) {
		Ok(program) => program,
		Err(e) => {
			error!("{}", e);
			exit(1);
		}
	}
}

/// Loads the shader `name` from the `shaders/` folder.
/// 
/// If it finds a file with the name of the shader and the extension
/// - `.vert` it will load it as a vertex shader
/// - `.frag` it will load it as a fragment shader
/// - TODO: More shader types
/// 
/// Returns an `Err` if the shader cannot be found or is invalid.
pub fn try_load_shader(ctx: &Rc<Context>, name: &str) -> Result<Program, String> {
	fn inner_try(ctx: &Rc<Context>, name: &str) -> Result<Program, String> {
		let base_dir = try_get_base_dir()?;
		
		let name = String::from(name);
		
		let shaders_dir = base_dir.join("shaders");
		assert_is_dir(&shaders_dir)?;
		
		let vert = try_read_file_string(shaders_dir.join(name.clone() + ".vert"))?;
		
		let frag = try_read_file_string(shaders_dir.join(name.clone() + ".frag"))?;
		
		debug!("Compiling shader '{}'...", name);
		match Program::from_source(ctx, &vert, &frag, None) {
			Ok(p) => Ok(p),
			Err(e) => Err(format!("compilation error:\n{}", e)),
		}
	}
	inner_try(ctx, name).map_err(|e| format!("cannot load shader '{}': {}", name, e))
}

/// Loads the font `name` at `index` from a file in the `fonts/` folder.
/// 
/// Exits if the font is not valid.
pub fn load_font(name: &str, index: usize) -> Font<'static> {
	match try_load_font(name, index) {
		Ok(font) => font,
		Err(e) => {
			error!("{}", e);
			exit(1);
		}
	}
}

/// Loads the font `name` at `index` from a file in the `fonts/` folder.
/// 
/// Returns an `Err` if the font is not valid.
pub fn try_load_font(name: &str, index: usize) -> Result<Font<'static>, String> {
	fn inner_try(name: &str, index: usize) -> Result<Font<'static>, String> {
		let base_dir = try_get_base_dir()?;
		let fonts_dir = base_dir.join("fonts");
		assert_is_dir(&fonts_dir)?;
		let font_path = fonts_dir.join(name);
		let bytes = try_read_file_bytes(&font_path)?;
		
		let collection = FontCollection::from_bytes(bytes)
			.map_err(|e| format!("invalid font: {}", e))?;
		collection.font_at(index)
			.map_err(|e| format!("invalid font at index {}: {}", index, e))
	}
	inner_try(name, index).map_err(|e| format!("cannot load font '{}': {}", name, e))
}

/// Loads the texture `name` from a file in the `textures/` folder and uploads it to OpenGL.
/// 
/// Exits if the texture could not be found, the texture was invalid, or it could not be uploaded to OpenGL.
pub fn load_texture(ctx: &Rc<Context>, name: &str) -> Texture2d {
	match try_load_texture(ctx, name) {
		Ok(texture) => texture,
		Err(e) => {
			error!("{}", e);
			exit(1);
		}
	}
}

/// Loads the texture `name` from a file in the `textures/` folder and uploads it to OpenGL.
/// 
/// Returns an `Err` if the texture could not be found, the texture was invalid, or it could not be uploaded to OpenGL.
pub fn try_load_texture(ctx: &Rc<Context>, name: &str) -> Result<Texture2d, String> {
	fn inner_try(ctx: &Rc<Context>, name: &str) -> Result<Texture2d, String> {
		let base_dir = try_get_base_dir()?;
		let textures_dir = base_dir.join("textures");
		assert_is_dir(&textures_dir)?;
		let texture_path = textures_dir.join(name);
		let bytes = try_read_file_bytes(&texture_path)?;
		
		let img = image::load_from_memory(&bytes).map_err(|e| format!("{}", e))?;
		let img_buffer = match img {
			DynamicImage::ImageLuma8(img)  => img.convert(),
			DynamicImage::ImageLumaA8(img) => img.convert(),
			DynamicImage::ImageRgb8(img)   => img.convert(),
			DynamicImage::ImageRgba8(img)  => img,
		};
		
		// Upload to OpenGL
		let dimensions = img_buffer.dimensions();
		let img = RawImage2d::from_raw_rgba(img_buffer.into_raw(), dimensions);
		Texture2d::new(ctx, img).map_err(|e| format!("{}", e))
	}
	inner_try(ctx, name).map_err(|e| format!("cannot load texture '{}': {}", name, e))
}
