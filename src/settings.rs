use std::default::Default;
use std::env::args_os;

pub struct Settings {
	pub verbose: bool,
}
impl Settings {
	pub fn from_args() -> Settings {
		let mut verbose = false;
		for arg in args_os() {
			if &arg == "-v" {
				verbose = true;
			}
		}
		Settings {
			verbose: verbose,
			.. Default::default()
		}
	}
}
impl Default for Settings {
	fn default() -> Settings {
		Settings {
			verbose: false,
		}
	}
}
