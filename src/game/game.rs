use std::time::{Duration, Instant};
use std::thread::sleep;

use glutin::Event;

use game::{GameState, KeyboardState};
use render::Render;
use util::DurationExt;

pub struct Game {
	render: Render,
	
	current_state: GameState,
	next_state: GameState,
	running: bool,
	keyboard_state: KeyboardState,
}
impl Game {
	pub fn new(render: Render) -> Game {
		Game::with_state(render, GameState::new())
	}

	pub fn with_state(render: Render, state: GameState) -> Game {
		Game {
			render: render,
			
			current_state: state.clone(),
			next_state: state,
			running: true,
			keyboard_state: KeyboardState::new(),
		}
	}
	
	pub fn main_loop(&mut self) {
		let mut last_render = Instant::now();
		while self.running {
			// Render to screen
			// TODO: Render using seperate thread (mutexes?).
			self.current_state.render(&mut self.render);
			
			let dt = last_render.elapsed();
			println!("{}ms", dt.as_millis());
			last_render = Instant::now();
			
			// Process events
			for e in self.render.poll_events() {
				match e {
					Event::Closed => {
						self.running = false;
						return; // Ignore all other events.
					}
					_ => self.keyboard_state.process_event(&e),
				}
			}
			
			// Tick game
			self.tick(dt.as_secs_partial() as f32);
			
			sleep(Duration::from_millis(10));
		}
	}
	
	/// Ticks the game. `dt` is the number of seconds since last frame.
	pub fn tick(&mut self, dt: f32) {
		// Tick next state
		self.next_state.tick(dt, &self.keyboard_state);

		// TODO: Wait for mutex on current state, as it might be being accessed by the renderer.
		self.current_state = self.next_state.clone();
	}
}
