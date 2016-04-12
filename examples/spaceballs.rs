extern crate neat;

use neat::game::GameStateBuilder;

pub fn main() {
	neat::run(GameStateBuilder::build_spaceballs);
}