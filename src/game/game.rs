use std::time::{Instant};
use std::rc::Rc;

use glutin::{VirtualKeyCode, Event, MouseButton, ElementState};
use na::Vec3;

use game::{GameState, KeyboardState, Entity};
use render::{Color, Render, SimpleMesh, ColoredMesh, Camera};
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
}
impl Game {
	pub fn new(settings: Settings, cam: Camera) -> Game {
		let mut render = Render::new(cam);
		info!("Initialized renderer");
		
		let state = {
			let sphere = Rc::new(SimpleMesh::sphere(render.context(), 4));
			let mut state = GameState::new(cam);
			//state.add_entity(Entity::new(Vec3::new(5.0, 0.0,  0.0), Vec3::new(0.0, 1.0, 0.0), 1.0, Color::RED  , sphere.clone()));
			//state.add_entity(Entity::new(Vec3::new(0.0, 0.0, -5.0), Vec3::new(1.0, 0.0, 0.0), 1.0, Color::GREEN, sphere.clone()));
			//state.add_entity(Entity::new(Vec3::new(0.0, 5.0,  0.0), Vec3::new(0.0, 0.0, 1.0), 1.0, Color::BLUE , sphere.clone()));
						
			let sun = Entity::new(Vec3::new( 0.0, 0.0, 0.0), Vec3::new(0.0, 0.0,  0.2), 100.0, Rc::new(ColoredMesh::new(sphere.clone(), Color::YELLOW)), false);
			state.add_entity(sun);
			
			let mut earth = Entity::new(Vec3::new(10.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -4.0), 5.0, Rc::new(ColoredMesh::new(sphere.clone(), Color::GREEN)), false);
			earth.scale(0.3684);
			state.add_entity(earth);
			
			let mut mercury = Entity::new(Vec3::new(2.5, 0.0, 0.0), Vec3::new(0.0, 0.0, -10.0), 0.05, Rc::new(ColoredMesh::new(sphere.clone(), Color::RED)), false);
			mercury.scale(0.07937);
			state.add_entity(mercury);
			state
		};
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
			focused: false,
		}
	}
	
	pub fn main_loop(&mut self) {
		info!("Starting game main loop");
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
				trace!("Event recieved: {:?}", e);
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
							// FIXME: Breaks when user clicks on title bar
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
						if key_state == ElementState::Pressed && Some(code) == self.settings.pause_key {
							self.settings.paused = !self.settings.paused;
							if self.settings.paused {
								info!("Game paused");
							} else {
								info!("Game resumed");
							}
						}
					},
					_ => {},
				}
			}
			
			if resized {
				info!("Resizing renderer");
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
		trace!("Game tick: {}s", dt);
		// Tick next state
		self.next_state.tick(dt, &self.settings, &self.keyboard_state, self.mouse_state);
		self.mouse_state = (0, 0);
		
		// TODO: Wait for mutex on current state, as it might be being accessed by the renderer.
		self.current_state = self.next_state.clone();
	}
}
