#![feature(box_syntax, question_mark, associated_consts, iter_arith, as_unsafe_cell, type_ascription)]
#[macro_use]
extern crate glium;
extern crate nalgebra as na;
extern crate ncollide as nc;
extern crate nphysics3d as np;
extern crate rusttype;
extern crate image;

#[macro_use]
extern crate log;
extern crate simplelog;
extern crate unicode_normalization;
#[macro_use]
extern crate cfg_if;

#[cfg(windows)]
extern crate user32;

use std::io::{self, Write, BufWriter};
use std::fs::File;
use std::rc::Rc;

use glium::backend::Context;
use simplelog::{CombinedLogger, TermLogger, WriteLogger, SharedLogger};

pub use glium::glutin;
pub mod render;
pub mod game;
pub mod util;
pub mod settings;
pub mod vfs;

use game::{Game, GameState};
use settings::Settings;

pub fn with_state<F>(generator: F) where F: FnOnce(&Rc<Context>) -> GameState {
	let settings = Settings::from_args();
	let mut loggers: Vec<Box<SharedLogger>> = Vec::new();
	let file_result = File::create(&settings.log_file);
	match file_result {
		Ok(f) => loggers.push(WriteLogger::new(settings.file_log_level, box BufWriter::new(f))),
		Err(e) => {let _ = writeln!(io::stderr(), "Error: Could not open log file '{}': {}", settings.log_file.display(), e); },
	}
	loggers.push(TermLogger::new(settings.term_log_level));
	CombinedLogger::init(loggers)
		.map_err(|e| writeln!(io::stderr(), "Error: Could not initialize logger: {}", e)).ok();
	
	info!("Initialized logger");
	
	let mut g = Game::with_state_generator(settings, generator);
	info!("Initialized game");
	
	g.main_loop();
	
	info!("Program exited");
}
