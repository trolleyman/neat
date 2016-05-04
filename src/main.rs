extern crate neat;

use neat::game::GameStateBuilder;

pub fn main() {
	neat::run(Box::new(GameStateBuilder::build_default));
}
