use std::path::{Path, PathBuf};
use std::fs;
use std::env;

#[cfg(windows)]
fn symlink_dir(src: &Path, dst: &Path) {
	::std::os::windows::fs::symlink_dir(src, dst).unwrap();
}

#[cfg(unix)]
fn symlink_dir(src: &Path, dst: &Path) {
	::std::os::unix::fs::symlink(src, dst).unwrap();
}

#[cfg(all(not(windows), not(unix)))]
fn symlink_dir(src: &Path, dst: &Path) {
	panic!("unsupported platform");
}

/// Recursively copy a folder
fn copy_folder(src: &Path, dst: &Path) {
	assert!(src.is_dir());
	if env::var("PROFILE").unwrap() == "release" {
		println!("cargo:rerun-if-changed={}", src.display());
		println!("Release mode -- copying all files to output dir");
		
		// Create dst dir
		fs::create_dir_all(dst).unwrap();
		assert!(dst.is_dir());
		println!("Created dir {}", dst.display());
		
		// Copy all files recursively
		for entry in fs::read_dir(src).unwrap() {
			let entry = entry.unwrap();
			println!("cargo:rerun-if-changed={}", entry.path().display());
			let ty = entry.file_type().unwrap();
			let dst = dst.join(entry.path().file_name().unwrap());
			if ty.is_dir() {
				copy_folder(&entry.path(), &dst);
			} else if ty.is_file() || ty.is_symlink() {
				fs::copy(&entry.path(), &dst).unwrap();
				println!("Copied '{}' => '{}'", entry.path().display(), dst.display());
			}
		}
	} else {
		println!("cargo:rerun-if-changed={}", src.display());
		println!("Debug mode -- symlinking");
		
		// Remove previous data
		if let Ok(meta) = fs::metadata(dst) {
			let ty = meta.file_type();
			if ty.is_dir() {
				fs::remove_dir_all(dst).unwrap();
			} else {
				fs::remove_file(dst).unwrap();
			}
		}
		symlink_dir(src, dst);
		println!("Symlinked '{}' => '{}'", dst.display(), src.display());
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
	copy_to_out("assets");
}
