use std::time::{Instant};

use glutin::{VirtualKeyCode, Event, MouseButton};

use game::{GameState, KeyboardState};
use render::Render;
use util::DurationExt;

pub struct Game {
	render: Render,
	
	current_state: GameState,
	next_state: GameState,
	running: bool,
	keyboard_state: KeyboardState,
	mouse_state: (i32, i32),
	focused: bool,
}
impl Game {
	pub fn new(render: Render) -> Game {
		let cam = render.camera().clone();
		Game::with_state(render, GameState::new(cam))
	}

	pub fn with_state(render: Render, state: GameState) -> Game {
		Game {
			render: render,
			
			current_state: state.clone(),
			next_state: state,
			running: true,
			keyboard_state: KeyboardState::new(),
			mouse_state: (0, 0),
			focused: true,
		}
	}
	
	pub fn main_loop(&mut self) {
		self.render.focus();
		
		let mut last_time = Instant::now();
		while self.running {
			// Render to screen
			// TODO: Render using seperate thread (mutexes?).
			
			let dt = last_time.elapsed();
			last_time = Instant::now();
			self.current_state.render(&mut self.render, dt);
			
			// Process events
			let (mp_x, mp_y) = self.render.get_window().and_then(|w| w.get_outer_size()).unwrap_or((0, 0));
			let (mp_x, mp_y) = (mp_x as i32 / 2, mp_y as i32 / 2);
			if self.focused {
				self.render.get_window().map(|w| w.set_cursor_position(mp_x, mp_y));
			}
			
			let mut resized = false;
			let mut focus = None;
			let mut mouse_pos = (mp_x, mp_y);
			for e in self.render.poll_events() {
				match e {
					Event::Closed => {
						self.running = false;
						return; // Ignore all other events.
					},
					Event::MouseMoved(pos) => {
						mouse_pos = pos;
					},
					Event::MouseInput(_mouse_state, button) => {
						if button == MouseButton::Left {
							self.render.get_window().map(|w| w.set_cursor_position(mp_x, mp_y));
							mouse_pos = (mp_x, mp_y);
							focus = Some(true);
						}
					},
					Event::Resized(_, _) => {
						resized = true;
					},
					Event::KeyboardInput(key_state, _, Some(code)) => {
						self.keyboard_state.process_event(key_state, code);
						if code == VirtualKeyCode::Escape {
							focus = Some(false);
						}
					},
					_ => {},
				}
			}
			
			if resized {
				// Resize and rerender.
				self.render.resize();
				self.current_state.render(&mut self.render, dt);
			}
			
			if let Some(focus) = focus {
				self.focused = focus;
				if focus {
					self.render.focus();
				} else {
					self.render.unfocus();
				}
			}
			
			if self.focused {
				let xdiff = mouse_pos.0 - mp_x;
				let ydiff = mouse_pos.1 - mp_y;
				//println!("mouse_pos: {:?}, mp_x: {}, mp_y: {}, xdiff: {}, ydiff: {}", mouse_pos, mp_x, mp_y, xdiff, ydiff);
				self.mouse_state = (xdiff, ydiff);
			} else {
				self.mouse_state = (0, 0);
			}
			
			// Tick game
			self.tick(dt.as_secs_partial() as f32);
			
			//sleep(Duration::from_millis(10));
		}
	}
	
	/// Ticks the game. `dt` is the number of seconds since last frame.
	pub fn tick(&mut self, dt: f32) {
		// Tick next state
		self.next_state.tick(dt, &self.keyboard_state, self.mouse_state);
		self.mouse_state = (0, 0);
		
		// TODO: Wait for mutex on current state, as it might be being accessed by the renderer.
		self.current_state = self.next_state.clone();
	}
}
