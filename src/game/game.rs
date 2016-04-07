use std::time::{Duration, Instant};

use glutin::{VirtualKeyCode, Event, MouseButton, ElementState};

use game::{GameState, KeyboardState};
use render::{Render, Camera};
use settings::Settings;
use util::DurationExt;

pub struct Game {
	render: Render,
	settings: Settings,
	
	current_state: GameState,
	next_state: GameState,
	running: bool,
	keyboard_state: KeyboardState,
	mouse_state: (i32, i32),
	focused: bool,
	step: bool,
	ignore_next_mouse_movement: bool,
}
impl Game {
	pub fn new(settings: Settings, cam: Camera) -> Game {
		let mut render = Render::new(cam);
		info!("Initialized renderer");
		
		let state = GameState::gen_balls(render.context(), cam);
		info!("Initialized game state");
		Game::with_state(settings, render, state)
	}

	pub fn with_state(settings: Settings, render: Render, state: GameState) -> Game {
		Game {
			render: render,
			settings: settings,
			
			current_state: state.clone(),
			next_state: state,
			running: true,
			keyboard_state: KeyboardState::new(),
			mouse_state: (0, 0),
			focused: true,
			step: false,
			ignore_next_mouse_movement: false,
		}
	}
	
	pub fn main_loop(&mut self) {
		// Try and focus on the game window
		self.focused = self.render.try_focus().is_ok();
		
		// How long each physics timestep should be.
		let sec = Duration::new(1, 0);
		let physics_dt = sec / 120;
		
		let mut lag = Duration::from_millis(0);
		let mut previous = Instant::now();
		
		let mut previous_fps_count = Instant::now();
		let mut frames = 0;
		let mut fps = 0;
		
		info!("Starting game main loop");
		while self.running {
			// Process timing stuff
			let current = Instant::now();
			let elapsed = current - previous;
			previous = current;
			lag += elapsed;
			
			// Calculate fps
			if current - previous_fps_count >= sec {
				previous_fps_count = Instant::now();
				fps = frames;
				frames = 0;
			}
			
			// Process events
			self.process_events();
			
			// Tick game
			let mut n = 0;
			while lag >= physics_dt {
				n += 1;
				lag -= physics_dt;
			}
			self.tick(physics_dt.as_secs_partial() as f32, n);
			
			// Render to screen
			// TODO: Render using seperate thread (mutexes?).
			self.current_state.render(&mut self.render, fps);
			frames += 1;
		}
	}
	
	pub fn process_events(&mut self) {
		let (mp_x, mp_y) = self.render.get_window().and_then(|w| w.get_outer_size()).unwrap_or((0, 0));
		let (mp_x, mp_y) = (mp_x as i32 / 2, mp_y as i32 / 2);
		if self.focused {
			self.render.get_window().map(|w| w.set_cursor_position(mp_x, mp_y));
		}
		
		if self.step {
			self.step = false;
			self.settings.paused = true;
		}
		
		let mut resized = false;
		let mut mouse_pos = (mp_x, mp_y);
		for e in self.render.poll_events() {
			trace!("Event recieved: {:?}", e);
			match e {
				Event::Closed => {
					self.running = false;
					return; // Ignore all other events.
				},
				Event::MouseMoved(pos) => {
					if self.ignore_next_mouse_movement {
						self.ignore_next_mouse_movement = false;
					} else {
						mouse_pos = pos;	
					}
				},
				Event::Focused(b) => {
					self.focused = b;
					if b {
						info!("Window focused");
					} else {
						info!("Window unfocused");
						self.focused = false;
					}
				},
				Event::MouseInput(mouse_state, button) => {
					if mouse_state == ElementState::Pressed && button == MouseButton::Left {
						if !self.focused {
							self.render.get_window().map(|w| w.set_cursor_position(mp_x, mp_y));
							mouse_pos = (mp_x, mp_y);
							self.ignore_next_mouse_movement = true;
							self.focused = true;
						}
					}
				},
				Event::Resized(_, _) => {
					resized = true;
				},
				Event::KeyboardInput(key_state, _, Some(code)) => {
					self.keyboard_state.process_event(key_state, code);
					if code == VirtualKeyCode::Escape {
						self.focused = false;
					}
					if key_state == ElementState::Pressed && Some(code) == self.settings.physics_pause {
						self.settings.paused = !self.settings.paused;
						if self.settings.paused {
							info!("Game paused");
						} else {
							info!("Game resumed");
						}
					}
					if key_state == ElementState::Pressed && Some(code) == self.settings.physics_step {
						if self.settings.paused {
							self.settings.paused = false;
							self.step = true;
							info!("Game stepped");
						}
					}
				},
				_ => {},
			}
		}
		
		if resized {
			info!("Resizing renderer");
			// Resize
			self.render.resize();
		}
		
		if self.focused {
			self.render.input_grab();
		} else {
			self.render.input_normal();
		}
		
		if self.focused /*&& !ignore_movement_frame*/ {
			let xdiff = mouse_pos.0 - mp_x;
			let ydiff = mouse_pos.1 - mp_y;
			//println!("mouse_pos: {:?}, mp_x: {}, mp_y: {}, xdiff: {}, ydiff: {}", mouse_pos, mp_x, mp_y, xdiff, ydiff);
			self.mouse_state = (xdiff, ydiff);
		} else {
			self.mouse_state = (0, 0);
		}
	}
	
	/// Ticks the game.
	/// `dt` is the number of seconds since last frame.
	/// `n` is the number of iterations to do.
	pub fn tick(&mut self, dt: f32, n: u32) {
		if n == 0 {
			return;
		}
		trace!("Game tick: {}s ({} iterations)", dt, n);
		// Tick next state
		for _ in 0..n {
			self.next_state.tick(dt, &self.settings, &self.keyboard_state, self.mouse_state);
			self.mouse_state = (0, 0);
		}
		
		// TODO: Wait for mutex on current state, as it might be being accessed by the renderer.
		self.current_state = self.next_state.clone();
	}
}
