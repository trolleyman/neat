extern crate nphysics3d as np;

use np::world::World;

pub fn main() {
	World::new().step(1.0);
}
