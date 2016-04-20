use std::time::{Duration, Instant};

use glutin::{VirtualKeyCode, Event, MouseButton, ElementState};

use game::{GameState, KeyboardState};
use render::{Render, Camera};
use settings::Settings;
use util::DurationExt;

pub struct Game {
}
impl Game {
	pub fn new(settings: Settings, cam: Camera) -> Game {
		let mut render = Render::new(cam);
		info!("Initialized renderer");
		
		let state = GameState::gen_balls(render.context(), cam);
		info!("Initialized game state");
		Game {
			
		}
	}
	
	pub fn main_loop(&mut self) {
		
	}
	
	/// Ticks the game.
	/// `dt` is the number of seconds since last frame.
	/// `n` is the number of iterations to do.
	pub fn tick(&mut self, dt: f32, n: u32) {
		
	}
}
