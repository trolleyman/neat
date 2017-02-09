use prelude::*;
use std::rc::Rc;
use std::thread::sleep;

use glutin::{VirtualKeyCode, Event, MouseButton, ElementState};

use game::{GameState, GameStateBuilder, KeyboardState};
use render::{Render, Camera};
use settings::Settings;

/// The structure that keeps track of game-wide state.
pub struct Game {
	render: Render,
	settings: Settings,
	
	state_generator: Box<Fn(&Rc<Context>) -> GameState>,
	current_state: GameState,
	keyboard_state: KeyboardState,
	running: bool,
	focused: bool,
	step: bool,
	ignore_next_mouse_movement: bool,
	skip_next_tick: bool,
	rerender: bool,
}
impl Game {
	/// Constructs a game with the specified settings, and the default game state.
	pub fn new(settings: Settings) -> Game {
		Game::with_state_generator(settings, box GameStateBuilder::build_default)
	}
	
	/// Cosnstructs a game with the specified settings, and a custom game state generator.
	pub fn with_state_generator<F>(settings: Settings, generator: Box<F>) -> Game where for<'r> F: Fn(&'r Rc<Context>) -> GameState + 'static {
		let mut render = Render::new(Camera::new(Vector3::new(0.0, 0.0, 0.0)), &settings);
		info!("Initialized renderer");
		
		let state = generator(render.context());
		render.set_camera(state.camera().clone());
		info!("Initialized game state");
		Game {
			render: render,
			settings: settings,
			
			state_generator: generator,
			current_state: state,
			keyboard_state: KeyboardState::new(),
			running: true,
			focused: true,
			step: false,
			ignore_next_mouse_movement: false,
			skip_next_tick: true,
			rerender: false,
		}
	}
	
	/// Performs the main loop.
	/// 
	/// This will only return when the user has exited the game.
	pub fn main_loop(&mut self) {
		// How long each physics timestep should be.
		const PHYSICS_HZ: u32 = 120;
		// Maximum lag in ms simulated.
		const MAX_LAG_MS: u64 = 500;
		let sec = Duration::new(1, 0);
		let physics_dt = sec / PHYSICS_HZ;
		
		// Minimum amount of time to wait between ticks
		let min_elapsed = Duration::from_millis(5);
		
		// Try and focus on the game window. If error, pause game.
		self.focused = self.render.try_focus().is_ok();
		if !self.focused {
			self.settings.paused = true;
		}
		
		let mut lag = Duration::from_millis(0);
		let mut previous = Instant::now();
		
		let mut previous_fps_count = Instant::now();
		let mut frames = 0;
		let mut fps = 0;
		
		self.render.show();
		let mut events = Vec::new();
		info!("Starting game main loop");
		while self.running {
			// Process timing stuff
			let mut current = Instant::now();
			let mut elapsed = current - previous;
			if elapsed < min_elapsed {
				sleep(min_elapsed - elapsed);
				current = Instant::now();
				elapsed = current - previous;
			}
			previous = current;
			lag += elapsed;
			
			// Make sure lag doesn't get bigger and bigger when the program is slow
			// This makes the simulation kinda non-deteministic
			if lag > Duration::from_millis(MAX_LAG_MS) {
				lag = Duration::from_millis(MAX_LAG_MS);
			}
			
			// Calculate fps
			if current - previous_fps_count >= sec {
				previous_fps_count = Instant::now();
				fps = frames;
				frames = 0;
			}
			
			// Process events
			events.clear();
			let mouse_moved = self.process_events(&mut events);
			if !self.running {
				break;
			}
			
			if self.skip_next_tick {
				lag = Duration::from_millis(0);
				self.skip_next_tick = false;
			}
			
			// Tick game
			if !self.rerender {
				let mut n = 0;
				while lag >= physics_dt {
					n += 1;
					lag -= physics_dt;
				}
				if n > 4 {
					warn!("Stutter detected ({}ms): {} iterations needed to catch up", elapsed.as_millis(), n);
				}
				if !self.skip_next_tick {
					self.tick(physics_dt.as_secs_partial() as f32, n, &mut events, mouse_moved);
				}
			} else {
				self.rerender = false;
			}
			
			if self.skip_next_tick {
				lag = Duration::from_millis(0);
				self.skip_next_tick = false;
			}
			
			// Render to screen
			// TODO: Render using seperate thread (mutexes?).
			self.current_state.render(&mut self.render, fps);
			frames += 1;
		}
	}
	
	/// Processes system events in the queue.
	/// 
	/// Appends events to pass onto the GameState to `events`
	/// 
	/// Returns how much the mouse has moved since the last frame.
	pub fn process_events(&mut self, events: &mut Vec<Event>) -> Vector2<i32> {
		// Find centre of screen.
		let mid = self.render.get_window().and_then(|w| w.get_outer_size()).unwrap_or((0, 0));
		let mid = Vector2::new(mid.0 as i32 / 2, mid.1 as i32 / 2);
		if self.focused {
			self.render.get_window().map(|w| w.set_cursor_position(mid.x, mid.y));
		}
		
		if self.step {
			self.step = false;
			self.settings.paused = true;
		}
		
		let mut reload_shaders = false;
		let mut rerender = false;
		let mut resized = false;
		let mut mouse_pos = mid;
		let ctx = self.render.context().clone();
		for e in self.render.poll_events() {
			// Filter out 'noisy' events from the log.
			let uninportant = match &e {
				&Event::MouseMoved(_, _) |
				&Event::Moved(_, _) => {
					true
				},
				&Event::KeyboardInput(ElementState::Pressed, _, Some(ref code))
					if self.keyboard_state.is_pressed(code) => {
						// Repeated key stroke
						true
				},
				_ => false,
			};
			
			if uninportant {
				trace!("Event recieved: {:?}", e);
			} else {
				debug!("Event recieved: {:?}", e);
			}
			
			let push = match &e {
				&Event::MouseMoved(_, _) => true,
				&Event::MouseInput(_, _) => true,
				&Event::MouseWheel(_, _) => true,
				&Event::KeyboardInput(_, _, _) => true,
				&Event::ReceivedCharacter(_) => true,
				_ => false,
			};
			if push {
				events.push(e.clone());
			}
			
			match e {
				Event::Closed => {
					self.running = false;
				},
				Event::MouseMoved(x, y) => {
					if self.ignore_next_mouse_movement {
						self.ignore_next_mouse_movement = false;
					} else {
						mouse_pos = Vector2::new(x, y);
					}
				},
				Event::Focused(b) => {
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
							self.render.get_window().map(|w| w.set_cursor_position(mid.x, mid.y));
							mouse_pos = mid;
							self.ignore_next_mouse_movement = true;
							self.focused = true;
						}
					}
				},
				Event::Resized(_, _) => {
					resized = true;
				},
				Event::Refresh => {
					rerender = true;
				},
				Event::KeyboardInput(key_state, _, Some(code)) => {
					self.keyboard_state.process_event(key_state, code);
					if key_state == ElementState::Pressed {
						if code == VirtualKeyCode::Escape {
							self.focused = false;
						} else if Some(code) == self.settings.physics_pause {
							self.settings.paused = !self.settings.paused;
							if self.settings.paused {
								info!("Game paused");
							} else {
								info!("Game resumed");
							}
						} else if Some(code) == self.settings.physics_step {
							if self.settings.paused {
								self.settings.paused = false;
								self.step = true;
								info!("Game stepped");
							}
						} else if Some(code) == self.settings.reload_shaders {
							reload_shaders = true;
						} else if Some(code) == self.settings.reset_state {
							info!("Resetting game state...");
							let sw = Stopwatch::start();
							self.current_state = (self.state_generator)(&ctx);
							info!("Reset game state ({}ms)", sw.elapsed_ms());
							self.skip_next_tick = true;
						}
					}
				},
				_ => {},
			}
		}
		
		// Reload shaders
		if reload_shaders {
			info!("Reloading shaders");
			let s = Stopwatch::start();
			match self.render.reload_shaders() {
				Ok(()) => info!("Reloaded shaders ({}ms)", s.elapsed_ms()),
				Err(e) => error!("Error reloading shaders: {}", e),
			}
			self.skip_next_tick = true;
		}
		
		if resized {
			debug!("Resizing renderer");
			rerender = true;
			// Resize
			self.render.resize();
		}
		
		self.rerender = rerender;
		
		if self.focused {
			self.render.input_grab();
		} else {
			self.render.input_normal();
		}
		
		if self.focused {
			mouse_pos - mid
		} else {
			Vector2::new(0, 0)
		}
	}
	
	/// Ticks the game.
	/// `dt` is the number of seconds since last frame.
	/// `n` is the number of iterations to do.
	pub fn tick(&mut self, dt: f32, n: u32, events: &mut Vec<Event>, mouse_moved: Vector2<i32>) {
		if n == 0 {
			return;
		}
		if n == 1 {
			trace!("Game tick: {}s ({} iteration)", dt, n);
		} else {
			trace!("Game tick: {}s ({} iterations)", dt, n);
		}
		// TODO: Interpolate mouse_moved.
		// TODO: Extension: Interpolate events.
		// Tick next state
		self.current_state.tick(dt, &self.settings, events, mouse_moved);
		for _ in 1..n {
			self.current_state.tick(dt, &self.settings, &mut Vec::with_capacity(0), Vector2::zero());
		}
	}
}
