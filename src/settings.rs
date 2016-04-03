use std::default::Default;
use std::env::args_os;

use glutin::VirtualKeyCode;

pub struct Settings {
	pub verbose : bool,
	pub forward : VirtualKeyCode,
	pub backward: VirtualKeyCode,
	pub left    : VirtualKeyCode,
	pub right   : VirtualKeyCode,
	pub up      : VirtualKeyCode,
	pub down    : VirtualKeyCode,
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
			forward : VirtualKeyCode::W,
			backward: VirtualKeyCode::S,
			left    : VirtualKeyCode::A,
			right   : VirtualKeyCode::D,
			up      : VirtualKeyCode::Q,
			down    : VirtualKeyCode::E,
		}
	}
}
