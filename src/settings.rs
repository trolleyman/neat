use std::default::Default;
use std::env::args;
use std::collections::HashSet;

use glutin::VirtualKeyCode;
use simplelog::LogLevelFilter;

pub struct Settings {
	pub w: u32,
	pub h: u32,
	pub vsync    : bool,
	pub paused   : bool,
	pub log_level: LogLevelFilter,
	pub forward  : VirtualKeyCode,
	pub backward : VirtualKeyCode,
	pub left     : VirtualKeyCode,
	pub right    : VirtualKeyCode,
	pub up       : VirtualKeyCode,
	pub down     : VirtualKeyCode,
	pub physics_pause: Option<VirtualKeyCode>,
	pub physics_step : Option<VirtualKeyCode>,
	pub wireframe_toggle: Option<VirtualKeyCode>,
}
impl Settings {
	/// Gets game settings from args passed to executable.
	/// Flags:
	///     -v | --verbose : Causes the game to be verbose
	///     -p | --paused  : The game will start paused.
	pub fn from_args() -> Settings {
		const LONG_START: &'static str = "--";
		const SHORT_START: &'static str = "-";
		
		// Args starting with '--'
		let mut long_args  = HashSet::<String>::new();
		// '-a' => {'a'}
		// '-a -b' => {'a', 'b'}
		// '-ab -c' => {'a', 'b', 'c'}
		let mut short_args = HashSet::<char>::new();
		// Other args
		let mut other_args = Vec::<String>::new();
		
		for arg in args().skip(1) {
			if arg.starts_with(LONG_START) {
				long_args.insert((&arg[LONG_START.len()..]).into());
			} else if arg.starts_with(SHORT_START) {
				for c in (&arg[SHORT_START.len()..]).chars() {
					short_args.insert(c);
				}
			} else {
				other_args.push(arg);
			}
		}
		
		println!("short_args: {:?}", short_args);
		println!("long_args : {:?}", long_args );
		println!("other_args: {:?}", other_args);
		
		Settings {
			paused   : short_args.contains(&'p'),
			vsync    : !long_args.contains("no-vsync"),
			log_level: if short_args.contains(&'V') {
				LogLevelFilter::Trace
			} else if short_args.contains(&'v') {
				LogLevelFilter::Debug
			} else {
				LogLevelFilter::Info
			},
			.. Default::default()
		}
	}
}
impl Default for Settings {
	fn default() -> Settings {
		Settings {
			w: 800,
			h: 600,
			vsync    : true,
			paused   : false,
			log_level: LogLevelFilter::Info,
			forward  : VirtualKeyCode::W,
			backward : VirtualKeyCode::S,
			left     : VirtualKeyCode::A,
			right    : VirtualKeyCode::D,
			up       : VirtualKeyCode::Q,
			down     : VirtualKeyCode::E,
			physics_pause: Some(VirtualKeyCode::F1),
			physics_step : Some(VirtualKeyCode::F2),
			wireframe_toggle: Some(VirtualKeyCode::F3),
		}
	}
}
