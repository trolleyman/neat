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
	
	pub fn add_entity(&mut self, e: Entity) {
		self.entities.push(e);
	}
	
	pub fn tick(&mut self, dt: f32) {
		// Apply gravity to all entities.
		/*
		for i in 0..self.entities.len() {
			let attractor = self.entities[i].clone();
			
			for j in i + 1..self.entities.len() {
				//const G: f64 = 6.674e-11;
				const G: f32 = 1.0;
				
				let mut o = &mut self.entities[j];
				// Get unit vector from o to attractor
				let mut v = attractor.pos() - o.pos();
				let len_sq = v.length2();
				v = v / len_sq.sqrt();
				
				// Apply a force towards the attractor.
				let f = v * ((G * attractor.weight() * o.weight()) / len_sq);
				o.force(f);
			}
		}*/

		// Collision check
		// TODO

		// Tick entities
		for e in &mut self.entities {
			e.tick(dt);
		}
	}

	pub fn render(&self, r: &mut Render) {
		self.entities.iter().map(|e| e.render(r)).count();
		
		r.swap();
	}
}
