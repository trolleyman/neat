use std::default::Default;
use std::env::args;
use std::collections::HashSet;
use std::path::PathBuf;

use glutin::VirtualKeyCode;
use simplelog::LogLevelFilter;

/// Game settings
pub struct Settings {
	/// Initial width of the window
	pub w: u32,
	/// Initial height of the window
	pub h: u32,
	/// If vsync is enabled
	pub vsync    : bool,
	/// If the game is currently paused
	pub paused   : bool,
	/// Where the log file will be located
	pub log_file : PathBuf,
	/// The log level for the terminal output
	pub term_log_level: LogLevelFilter,
	/// The log level for the file output
	pub file_log_level: LogLevelFilter,
	/// Forwards key
	pub forward  : VirtualKeyCode,
	/// Backwards key
	pub backward : VirtualKeyCode,
	/// Strafe left key
	pub left     : VirtualKeyCode,
	/// Strafe right key
	pub right    : VirtualKeyCode,
	/// Move up key
	pub up       : VirtualKeyCode,
	/// Move down key
	pub down     : VirtualKeyCode,
	/// They key to pause/resume the simulation
	pub physics_pause: Option<VirtualKeyCode>,
	/// The key to step the simulation
	pub physics_step : Option<VirtualKeyCode>,
	/// The key to toggle wireframe mode
	pub wireframe_toggle: Option<VirtualKeyCode>,
}
impl Settings {
	/// Gets game settings from args passed to executable.
	/// 
	/// # Usage
	/// - `-v` : Causes the game to be verbose
	/// - `-p` : The game will start paused.
	pub fn from_args() -> Settings {
		const LONG_START: &'static str = "--";
		const SHORT_START: &'static str = "-";
		
		// Args starting with '--'
		let mut long_args  = HashSet::<String>::new();
		// '-a' => {'a'}
		// '-a -b' => {'a', 'b'}
		// '-ab -c' => {'a', 'b', 'c'}
		let mut short_args = HashSet::<char>::new();
		// Other args, in order
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
		
		let (term_log_level, file_log_level) = if short_args.contains(&'V') {
				(LogLevelFilter::Trace, LogLevelFilter::Trace)
			} else if short_args.contains(&'v') {
				(LogLevelFilter::Debug, LogLevelFilter::Trace)
			} else {
				(<Settings as Default>::default().term_log_level, <Settings as Default>::default().file_log_level)
			};
		
		Settings {
			paused   : short_args.contains(&'p'),
			vsync    : !long_args.contains("no-vsync"),
			term_log_level: term_log_level,
			file_log_level: file_log_level,
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
			log_file : PathBuf::from("log.txt"),
			term_log_level: LogLevelFilter::Info,
			file_log_level: LogLevelFilter::Debug,
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
