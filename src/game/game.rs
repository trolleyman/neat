use std::default::Default;
use std::mem;
use std::iter::Iterator;

use glutin::Event;

use super::GameState;

pub struct Game {
	current_state: GameState,
	next_state: GameState,
	running: bool,
}
impl Game {
	pub fn new() -> Game {
		Game::with_state(Default::default())
	}

	pub fn with_state(state: GameState) -> Game {
		Game {
			current_state: state,
			next_state: GameState::new(),
			running: true,
		}
	}

	pub fn running(&self) -> bool {
		return self.running;
	}

	pub fn current_state(&self) -> &GameState {
		&self.current_state
	}

	/// Ticks the game. `dt` is the number of seconds since last frame.
	pub fn tick<I: Iterator<Item = Event>>(&mut self, dt: f64, events: I) {
		// Clone current state
		self.next_state = self.current_state.clone();

		// Apply events
		for e in events {
			match e {
				Event::Closed => {
					self.running = false;
					return; // Ignore all other events.
				}
				_ => {}//self.next_state.handle_event(e),
			}
		}

		// Tick next state
		self.next_state.tick(dt);

		// TODO: Wait for mutex on current state, as it might be being accessed by the renderer.
		mem::swap(&mut self.next_state, &mut self.current_state);
	}
}
