use std::collections::HashMap;

use glutin::{VirtualKeyCode, ElementState};

pub struct KeyboardState {
	state: HashMap<VirtualKeyCode, ElementState>,
}
impl KeyboardState {
	pub fn new() -> KeyboardState {
		KeyboardState {
			state: HashMap::new(),
		}
	}
	
	pub fn is_pressed(&self, key: &VirtualKeyCode) -> bool {
		self.state.get(key).map(|&s| s == ElementState::Pressed).unwrap_or(false)
	}
	pub fn is_released(&self, key: &VirtualKeyCode) -> bool {
		!self.is_pressed(key)
	}
	
	pub fn process_event(&mut self, key_state: ElementState, code: VirtualKeyCode) {
		self.state.insert(code, key_state);
	}
}
