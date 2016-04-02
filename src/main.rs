#![feature(box_syntax, question_mark, associated_consts)]
extern crate rusttype;
#[macro_use]
extern crate glium;
extern crate cgmath as cgmath;

use std::io::{self, Write};
use std::fmt::Display;
use std::rc::Rc;

pub use glium::glutin;

use cgmath::vec3;

pub mod render;
pub mod game;
pub mod util;

use render::{Camera, Mesh, Color, Render};
use game::{Entity, Game, GameState};

pub fn error<E: Display>(e: E) -> ! {
	// TODO: MsgBox
	writeln!(io::stderr(), "Error: {}", e).ok();
	::std::process::exit(1);
}

fn main() {
	let cam = Camera::new(vec3(2.0, 2.0, 10.0));
	let mut r = Render::new(cam);
	
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
	let mut g = Game::with_state(r, state);
	
	g.main_loop();

	println!("Program exited.");
}
