#![feature(box_syntax, question_mark)]
extern crate rusttype;
extern crate glium;

use std::time::Instant;
use std::io::{self, Write};
use std::fmt::Display;

pub use glium::glutin;

pub mod render;
pub mod game;
pub mod math;
pub mod util;

use util::DurationExt;

use render::Render;
use game::Game;

pub fn error<E: Display>(e: E) -> ! {
	// TODO: MsgBox
	writeln!(io::stderr(), "Error: {}", e).ok();
	::std::process::exit(1);
}

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
		g.tick(dt.as_secs_partial(), r.poll_events());
	}

	println!("Program exited.");
}
