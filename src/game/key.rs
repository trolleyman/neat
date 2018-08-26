use std::collections::HashSet;

use glutin::{VirtualKeyCode, ElementState};

/// Keeps track of which keys have been pressed.
pub struct KeyboardState {
	pressed: HashSet<VirtualKeyCode>,
}
impl KeyboardState {
	/// Constructs a new KeyboardState with all the keys released.
	pub fn new() -> KeyboardState {
		KeyboardState {
			pressed: HashSet::new(),
		}
	}
	
	/// Returns true if `key` is pressed.
	pub fn is_pressed(&self, key: &VirtualKeyCode) -> bool {
		self.pressed.contains(key)
	}
	/// Returns true if `key` is released.
	pub fn is_released(&self, key: &VirtualKeyCode) -> bool {
		!self.is_pressed(key)
	}
	
	/// Processes a keyboard event and updated the internal state.
	pub fn process_event(&mut self, key_state: ElementState, code: VirtualKeyCode) {
		match key_state {
			ElementState::Pressed => {
				self.pressed.insert(code);
			},
			ElementState::Released => {
				self.pressed.remove(&code);
			}
		}
	}
}
