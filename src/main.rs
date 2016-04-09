#![feature(box_syntax, question_mark, associated_consts, iter_arith)]
#[macro_use]
extern crate glium;
extern crate nalgebra as na;
extern crate ncollide as nc;
extern crate nphysics3d as np;
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
use na::Vec3;
use simplelog::{TermLogger, LogLevelFilter};

pub mod render;
pub mod game;
pub mod util;
pub mod settings;
pub mod vfs;

use render::Camera;
use game::Game;
use settings::Settings;

pub fn main() {
	let settings = Settings::from_args();
	let log_level = if settings.verbose { LogLevelFilter::Debug } else { LogLevelFilter::Info };
	TermLogger::init(log_level)
		.map_err(|e| writeln!(io::stderr(), "Error: Could not initialize logger: {}", e)).ok();
	info!("Initialized logger");
	
	let mut g = Game::new(settings, Camera::new(Vec3::new(2.0, 2.0, 10.0)));
	info!("Initialized game");
	
	g.main_loop();

	info!("Program exited");
}
