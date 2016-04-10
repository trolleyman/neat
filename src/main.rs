extern crate neat;

use neat::game::GameState;

pub fn main() {
	neat::with_state(|ctx| GameState::gen_ball_upside_down_pyramid(ctx));
}