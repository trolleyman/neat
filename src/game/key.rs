use std::collections::HashMap;

use glutin::{VirtualKeyCode, ElementState};

/// Keeps track of which keys have been pressed.
pub struct KeyboardState {
	state: HashMap<VirtualKeyCode, ElementState>,
}
impl KeyboardState {
	/// Constructs a new KeyboardState with all the keys released.
	pub fn new() -> KeyboardState {
		KeyboardState {
			state: HashMap::new(),
		}
	}
	
	/// Returns true if either `ctrl` key is pressed.
	pub fn is_ctrl_pressed(&self) -> bool {
		self.is_pressed(&VirtualKeyCode::LControl) || self.is_pressed(&VirtualKeyCode::RControl)
	}
	
	/// Returns true if either `shift` key is pressed.
	pub fn is_shift_pressed(&self) -> bool {
		self.is_pressed(&VirtualKeyCode::LShift) || self.is_pressed(&VirtualKeyCode::RShift)
	}
	
	/// Returns true if either `menu` key is pressed.
	pub fn is_menu_pressed(&self) -> bool {
		self.is_pressed(&VirtualKeyCode::LMenu) || self.is_pressed(&VirtualKeyCode::RMenu)
	}
	
	/// Returns true if `key` is pressed.
	pub fn is_pressed(&self, key: &VirtualKeyCode) -> bool {
		self.state.get(key).map(|&s| s == ElementState::Pressed).unwrap_or(false)
	}
	/// Returns true if `key` is released.
	pub fn is_released(&self, key: &VirtualKeyCode) -> bool {
		!self.is_pressed(key)
	}
	
	/// Processes a keyboard event and updated the internal state.
	pub fn process_event(&mut self, key_state: ElementState, code: VirtualKeyCode) {
		match key_state {
			ElementState::Pressed => {
				self.state.insert(code, key_state);
			},
			ElementState::Released => {
				self.state.remove(&code);
			}
		}
	}
}
