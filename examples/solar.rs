extern crate neat;

pub fn main() {
	neat::with_state(neat::game::GameState::gen_solar);
}