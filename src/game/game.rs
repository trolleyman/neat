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
	
	current_state: GameState,
	running: bool,
	keys: Vec<(ElementState, VirtualKeyCode)>,
	keyboard_state: KeyboardState,
	mouse_state: (i32, i32),
	focused: bool,
	step: bool,
	ignore_next_mouse_movement: bool,
}
impl Game {
	/// Constructs a game with the specified settings, and the default game state.
	pub fn new(settings: Settings) -> Game {
		Game::with_state_generator(settings, GameStateBuilder::build_default)
	}
	
	/// Cosnstructs a game with the specified settings, and a custom game state generator.
	pub fn with_state_generator<F>(settings: Settings, generator: F) -> Game where F: FnOnce(&Rc<Context>) -> GameState {
		let mut render = Render::new(Camera::new(Vec3::new(0.0, 0.0, 0.0)), &settings);
		info!("Initialized renderer");
		
		let state = generator(render.context());
		render.set_camera(state.camera().clone());
		info!("Initialized game state");
		Game::with_state(settings, render, state)
	}
	
	/// Constructs a game with the specified settings, renderer, and initial game state.
	pub fn with_state(settings: Settings, render: Render, state: GameState) -> Game {
		Game {
			render: render,
			settings: settings,
			
			current_state: state,
			running: true,
			keys: Vec::new(),
			keyboard_state: KeyboardState::new(),
			mouse_state: (0, 0),
			focused: true,
			step: false,
			ignore_next_mouse_movement: false,
		}
	}
	
	/// Performs the main loop.
	/// 
	/// This will only return when the user has exited the game.
	pub fn main_loop(&mut self) {
		// How long each physics timestep should be.
		const PHYSICS_HZ: u32 = 120;
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
			if n > 4 {
				warn!("Stutter detected ({}ms): {} iterations needed to catch up", elapsed.as_millis(), n);
			}
			self.tick(physics_dt.as_secs_partial() as f32, n);
			
			// Render to screen
			// TODO: Render using seperate thread (mutexes?).
			self.current_state.render(&mut self.render, fps);
			frames += 1;
		}
	}
	
	/// Processes system events in the queue.
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
		
		let mut reload_shaders = false;
		let mut resized = false;
		let mut mouse_pos = (mp_x, mp_y);
		let ctx = self.render.context().clone();
		for e in self.render.poll_events() {
			
			// Filter out 'noisy' events
			let uninportant = match &e {
				&Event::MouseMoved(_) |
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
					} else if key_state == ElementState::Pressed && Some(code) == self.settings.physics_step {
						if self.settings.paused {
							self.settings.paused = false;
							self.step = true;
							info!("Game stepped");
						}
					} else if key_state == ElementState::Pressed && Some(code) == self.settings.reload_shaders {
						reload_shaders = true;
					} else {
						self.keys.push((key_state, code));
					}
				},
				Event::ReceivedCharacter(c) => if self.settings.dev {
					let gen = match c {
						'1' => GameStateBuilder::build_default,
						'2' => GameStateBuilder::build_solar,
						'3' => GameStateBuilder::build_rot_test,
						'4' => GameStateBuilder::build_spaceballs,
						'5' => GameStateBuilder::build_balls,
						'6' => GameStateBuilder::build_phong,
						'7' => GameStateBuilder::build_tables,
						_ => continue,
					};
					self.current_state = gen(&ctx);
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
		}
		
		if resized {
			debug!("Resizing renderer");
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
		if n == 1 {
			trace!("Game tick: {}s ({} iteration)", dt, n);
		} else {
			trace!("Game tick: {}s ({} iterations)", dt, n);
		}
		// Tick next state
		for _ in 0..n {
			self.current_state.tick(dt, &self.settings, &self.keys, &self.keyboard_state, self.mouse_state);
			self.keys.clear();
			self.mouse_state = (0, 0);
		}
	}
}
