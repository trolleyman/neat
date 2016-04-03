#![feature(box_syntax, question_mark, associated_consts)]
#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate rusttype;

#[macro_use]
extern crate log;
extern crate simplelog;
extern crate unicode_normalization;
#[macro_use]
extern crate cfg_if;

#[cfg(windows)]
extern crate user32;

use std::io::{self, Write};

pub use glium::glutin;
use cgmath::vec3;
use simplelog::{TermLogger, LogLevelFilter};

pub mod render;
pub mod game;
pub mod util;
pub mod collision;
pub mod settings;
pub mod vfs;

use render::Camera;
use game::Game;
use settings::Settings;

fn main() {
	let settings = Settings::from_args();
	let log_level = if settings.verbose { LogLevelFilter::Debug } else { LogLevelFilter::Info };
	TermLogger::init(log_level)
		.map_err(|e| writeln!(io::stderr(), "Error: Could not initialize logger: {}", e)).ok();
	info!("Initialized logger");
	
	let mut g = Game::new(settings, Camera::new(vec3(2.0, 2.0, 10.0)));
	info!("Initialized game");
	
	g.main_loop();

	info!("Program exited");
}
