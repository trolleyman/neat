#![feature(box_syntax)]
extern crate rusttype;
extern crate glium;

use std::time::Instant;

pub use glium::glutin;

pub mod render;
pub mod game;
pub mod math;
pub mod util;

use util::DurationExt;

use render::Render;
use game::Game;

fn main() {
	let mut r = Render::new();
	let mut g = Game::new();

	let mut last_render = Instant::now();
	while g.running() {
		// Render to screen
		// TODO: Render using seperate thread (mutexes?).
		g.current_state().render(&mut r);

		let dt = last_render.elapsed();
		println!("{}ms", dt.as_millis());
		last_render = Instant::now();

		// Process events
		g.tick(dt.as_secs_partial() as f32, r.poll_events());
	}

	println!("Program exited.");
}
