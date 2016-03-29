use std::rc::Rc;

use glium::backend::{Context, Facade};
use glium::backend::glutin_backend::{GlutinFacade, PollEventsIter};
use glium::{DisplayBuild, Frame, Surface};

use glutin::WindowBuilder;

/// Render handler.
pub struct Render {
	win: GlutinFacade,
	_context: Rc<Context>,
	frame: Frame,
}
impl Render {
	pub fn new() -> Render {
		Render::with_size(800, 600)
	}

	pub fn with_size(w: u32, h: u32) -> Render {
		let win = WindowBuilder::new()
			          .with_dimensions(w, h)
			          .with_title("NEAT".into())
			          .build_glium()
			          .unwrap();

		let mut frame = win.draw();
		frame.clear_color(0.0, 0.0, 0.0, 0.0);
		frame.finish().ok();

		let frame = win.draw();

		let ctx = win.get_context().clone();

		Render {
			win: win,
			_context: ctx,
			frame: frame,
		}
	}

	pub fn poll_events<'a>(&'a self) -> PollEventsIter<'a> {
		self.win.poll_events()
	}

	pub fn draw(&mut self) -> &mut Frame {
		&mut self.frame
	}

	pub fn swap(&mut self) {
		self.frame.set_finish().ok();
		self.frame = self.win.draw();
	}
}

impl Drop for Render {
	fn drop(&mut self) {
		self.frame.set_finish().ok();
	}
}
