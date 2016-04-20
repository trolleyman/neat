
use game::{GameState, KeyboardState};
use render::{Render, Camera};
use util::DurationExt;

pub struct Game {
}
impl Game {
	pub fn new(cam: Camera) -> Game {
		let mut render = Render::new(cam);
		
		let state = GameState::gen_balls(render.context(), cam);
		Game {
			
		}
	}
}
