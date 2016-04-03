use std::default::Default;
use std::env::args_os;

use glutin::VirtualKeyCode;

pub struct Settings {
	pub verbose  : bool,
	pub paused   : bool,
	pub forward  : VirtualKeyCode,
	pub backward : VirtualKeyCode,
	pub left     : VirtualKeyCode,
	pub right    : VirtualKeyCode,
	pub up       : VirtualKeyCode,
	pub down     : VirtualKeyCode,
	pub pause_key: Option<VirtualKeyCode>,
}
impl Settings {
	/// Gets game settings from args passed to executable.
	/// Flags:
	///     -v | --verbose : Causes the game to be verbose
	///     -p | --paused  : The game will start paused.
	pub fn from_args() -> Settings {
		let mut verbose = false;
		let mut paused  = false;
		
		for arg in args_os() {
			if &arg == "--verbose" || &arg == "-v" {
				verbose = true;
			} else if &arg == "--paused" || &arg == "-p" {
				paused = true;
			}
		}
		Settings {
			verbose: verbose,
			paused : paused,
			.. Default::default()
		}
	}
}
impl Default for Settings {
	fn default() -> Settings {
		Settings {
			verbose  : false,
			paused   : false,
			forward  : VirtualKeyCode::W,
			backward : VirtualKeyCode::S,
			left     : VirtualKeyCode::A,
			right    : VirtualKeyCode::D,
			up       : VirtualKeyCode::Q,
			down     : VirtualKeyCode::E,
			pause_key: Some(VirtualKeyCode::F4),
		}
	}
}
