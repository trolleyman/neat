use std::path::{Path, PathBuf};
use std::fs;
use std::env;

/// Recursively copy a folder
fn copy_folder(src: &Path, dst: &Path) {
	assert!(src.is_dir());
	fs::create_dir_all(dst).unwrap();
	assert!(dst.is_dir());
	for entry in fs::read_dir(src).unwrap() {
		let entry = entry.unwrap();
		let ty = entry.file_type().unwrap();
		let dst = dst.join(entry.path().file_name().unwrap());
		if ty.is_dir() {
			copy_folder(&entry.path(), &dst);
		} else if ty.is_file() {
			fs::copy(&entry.path(), &dst).unwrap();
			println!("Copied '{}' => '{}'", entry.path().display(), dst.display());
		} // Ignore symbolic links
	}
}

fn copy_to_out(name: &str) {
	let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
	let out_dir = manifest_dir.join("target").join(env::var("PROFILE").unwrap());
	// Hack to get examples to work
	copy_folder(&manifest_dir.join(name), &out_dir.join(name));
	copy_folder(&manifest_dir.join(name), &out_dir.join("examples").join(name));
}

pub fn main() {
	fn print_env(s: &str) {
		println!("{}: {}", s, env::var(s).unwrap());
	}
	print_env("CARGO_MANIFEST_DIR");
	print_env("OUT_DIR");
	print_env("TARGET");
	print_env("HOST");
	print_env("NUM_JOBS");
	print_env("OPT_LEVEL");
	print_env("PROFILE");
	print_env("DEBUG");
	println!("");
	copy_to_out("fonts");
	copy_to_out("shaders");
	copy_to_out("textures");
}
