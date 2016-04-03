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
use std::rc::Rc;

pub use glium::glutin;
use cgmath::vec3;
use simplelog::{TermLogger, LogLevelFilter};

pub mod render;
pub mod game;
pub mod util;
pub mod collision;
pub mod settings;

use render::{Camera, Mesh, Color, Render};
use game::{Entity, Game, GameState};
use settings::Settings;

fn main() {
	let settings = Settings::from_args();
	let log_level = if settings.verbose { LogLevelFilter::Debug } else { LogLevelFilter::Info };
	TermLogger::init(log_level)
		.map_err(|e| writeln!(io::stderr(), "Error: Could not initialize logger: {}", e)).ok();
	info!("Initialized logger");
	
	let cam = Camera::new(vec3(2.0, 2.0, 10.0));
	let mut r = Render::new(cam);
	info!("Initialized renderer");
	
	let state = {
		let sphere = Rc::new(Mesh::sphere(r.context(), 4));
		let mut state = GameState::new(cam);
		//state.add_entity(Entity::new(vec3(5.0, 0.0,  0.0), vec3(0.0, 1.0, 0.0), 1.0, Color::RED  , sphere.clone()));
		//state.add_entity(Entity::new(vec3(0.0, 0.0, -5.0), vec3(1.0, 0.0, 0.0), 1.0, Color::GREEN, sphere.clone()));
		//state.add_entity(Entity::new(vec3(0.0, 5.0,  0.0), vec3(0.0, 0.0, 1.0), 1.0, Color::BLUE , sphere.clone()));
		
		state.add_entity(Entity::new(vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0,  0.2), 100.0, Color::YELLOW, sphere.clone())); // Sun
		state.add_entity(Entity::new(vec3(10.0, 0.0, 0.0), vec3(0.0, 0.0, -4.0), 5.0, Color::GREEN, sphere.clone())); // Earth
		state
	};
	info!("Initialized game state");
	let mut g = Game::with_state(settings, r, state);
	info!("Initialized game");
	
	g.main_loop();

	info!("Program exited.");
}
