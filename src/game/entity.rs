use na::{Iso3, ToHomogeneous};
use np::volumetric::Volumetric;

use render::{Render, RenderableMesh};

pub struct Entity {
	/// Visible mesh
	mesh: Box<RenderableMesh>,
	/// Density
	mass: f32,
}
impl Entity {
	/// Constructs an entity and adds it to `world`, returning it's ID
	pub fn new(mesh: Box<RenderableMesh>, mass: f32) -> Entity {
		Entity {
			mesh: mesh,
			mass: mass,
		}
	}
	
	/// Returns the mass of the object
	pub fn mass(&self) -> f32 {
		self.mass
	}
	
	/// Renders the entity
	pub fn render(&self, r: &mut Render, position: Iso3<f32>) {
		self.mesh.render(r, position.to_homogeneous());
	}
}
