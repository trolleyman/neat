extern crate neat;

use std::process::exit;
use std::io::{self, Write};

use neat::game::GameStateBuilder;

pub fn main() {
	match neat::run(Box::new(GameStateBuilder::build_balls)) {
		Ok(()) => {},
		Err(e) => {
			writeln!(io::stderr(), "Error: {}", e).ok();
			exit(1);
		}
	}
}