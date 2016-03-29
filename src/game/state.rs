use game::Entity;
use render::Render;

#[derive(Default, Clone)]
pub struct State {
	entities: Vec<Entity>,
}
impl State {
	pub fn new() -> State {
		State { entities: Vec::new() }
	}

	pub fn tick(&mut self, dt: f32) {
		// Apply gravity to all entities.
		// TODO

		// Collision check
		// TODO

		// Tick entities
		for e in &mut self.entities {
			e.tick(dt);
		}
	}

	pub fn render(&self, _r: &mut Render) {
		// Do nothing atm.
	}
}
