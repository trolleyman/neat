extern crate neat;

use neat::game::GameStateBuilder;

pub fn main() {
	neat::with_state(GameStateBuilder::build_balls);
}