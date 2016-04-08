use std::rc::Rc;
use std::cell::RefCell;

use na::{Pnt3, Iso3, ToHomogeneous};
use np::object::{RigidBody, RigidBodyHandle};
use np::volumetric::Volumetric;
use np::detection::joint::{Anchor, Fixed, BallInSocket};
use np::world::World;

use game::GameState;
use render::{Render, RenderableMesh};

pub struct Component {
	body: RigidBody<f32>,
	mesh: Rc<RenderableMesh>,
}
impl Component {
	pub fn new(body: RigidBody<f32>, mesh: Rc<RenderableMesh>) -> Component {
		Component {
			body: body,
			mesh: mesh,
		}
	}
}

pub struct ComponentHandle {
	body: RigidBodyHandle<f32>,
	mesh: Rc<RenderableMesh>,
}

/// Fixed joint, using Ids
pub struct FixedIds {
	a: ComponentId,
	a_pos: Iso3<f32>,
	b: ComponentId,
	b_pos: Iso3<f32>,
}
impl FixedIds {
	pub fn new(a: ComponentId, a_pos: Iso3<f32>, b: ComponentId, b_pos: Iso3<f32>) -> FixedIds {
		FixedIds {
			a: a,
			a_pos: a_pos,
			b: b,
			b_pos: b_pos,
		}
	}
}

/// Ball in socket joint, using Ids
pub struct BallInSocketIds {
	a: ComponentId,
	a_pos: Pnt3<f32>,
	b: ComponentId,
	b_pos: Pnt3<f32>,
}
impl BallInSocketIds {
	pub fn new(a: ComponentId, a_pos: Pnt3<f32>, b: ComponentId, b_pos: Pnt3<f32>) -> BallInSocketIds {
		BallInSocketIds {
			a: a,
			a_pos: a_pos,
			b: b,
			b_pos: b_pos,
		}
	}
}

/// ID of the root component in an entity.
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Debug)]
struct ComponentId(usize);
pub const ROOT_ID: ComponentId = ComponentId(0);

pub struct EntityBuilder {
	components: Vec<Component>,
	fixed_joints: Vec<FixedIds>,
	ball_joints: Vec<BallInSocketIds>,
}
impl EntityBuilder {
	/// Creates a new EntityBuilder with a root component.
	pub fn new(root: Component) -> EntityBuilder {
		EntityBuilder {
			components  : vec![root],
			fixed_joints: Vec::new(),
			ball_joints : Vec::new(),
		}
	}
	
	/// Adds a component that is fixed to a component that has already been added to the entity.
	/// 
	/// # Params
	/// `a_id` is the ID of the component to be fixed to.
	/// `a_pos` is the position, in co-ordinates local to `a`, of the joint.
	/// `b` is the component to be added to the Entity.
	/// `b_pos` is the position, in co-ordinates local to `b` of the joint.
	/// 
	/// # Returns
	/// The ID of the component that has been added.
	///
	/// # Panics
	/// If the ID is not valid.
	pub fn add_fixed(&mut self, a_id: ComponentId, a_pos: Iso3<f32>, b: Component, b_pos: Iso3<f32>) -> ComponentId {
		if a_id >= ComponentId(self.components.len()) {
			panic!("a_id {:?} is not valid.", a_id);
		}
		
		let b_id = ComponentId(self.components.len());
		self.fixed_joints.push(FixedIds::new(a_id, a_pos, b_id, b_pos));
		b_id
	}
	
	/// Adds a component that is fixed to a component that has already been added to the entity by a ball in socket joint.
	/// 
	/// # Params
	/// `a_id` is the ID of the component to be fixed to.
	/// `a_pos` is the position, in co-ordinates local to `a`, of the joint.
	/// `b` is the component to be added to the Entity.
	/// `b_pos` is the position, in co-ordinates local to `b` of the joint.
	/// 
	/// # Returns
	/// The ID of the component that has been added.
	///
	/// # Panics
	/// If the ID is not valid.
	pub fn add_ball_in_socket(&mut self, a_id: ComponentId, a_pos: Iso3<f32>, b: Component, b_pos: Iso3<f32>) -> ComponentId {
		if a_id >= ComponentId(self.components.len()) {
			panic!("a_id {:?} is not valid.", a_id);
		}
		
		let b_id = ComponentId(self.components.len());
		self.fixed_joints.push(FixedIds::new(a_id, a_pos, b_id, b_pos));
		b_id
	}
	
	/// Builds the entity by adding it to a GameState.
	pub fn build(self, state: &mut GameState) {
		state.add_entity(self);
	}
	
	/// Builds the entity by adding it to the world.
	pub fn build_world(self, world: &mut World<f32>) -> Entity {
		Entity::with_joints(world, self.components, self.fixed_joints, self.ball_joints)
	}
}

pub struct Entity {
	mass: f32,
	components: Vec<ComponentHandle>,
	fixed_joints: Vec<Rc<RefCell<Fixed<f32>>>>,
	ball_joints: Vec<Rc<RefCell<BallInSocket<f32>>>>,
}
impl Entity {
	pub fn new(world: &mut World<f32>, component: Component) -> Entity {
		Entity::with_joints(world, vec![component], Vec::new(), Vec::new())
	}
	
	pub fn with_joints(world: &mut World<f32>, components: Vec<Component>, fixed_joints: Vec<FixedIds>, ball_joints: Vec<BallInSocketIds>) -> Entity {
		let components: Vec<_> = components.drain(..).map(|c| {
			ComponentHandle {
				body: world.add_body(c.body),
				mesh: c.mesh,
			}
		}).collect();
		
		let fixed_joints = fixed_joints.drain(..).map(|j| {
			let a = components[j.a.0].body;
			let b = components[j.b.0].body;
			Fixed::new(Anchor::new(Some(a), j.a_pos), Anchor::new(Some(b), j.b_pos))
		}).collect();
		
		let ball_joints = ball_joints.drain(..).map(|j| {
			let a = components[j.a.0].body;
			let b = components[j.b.0].body;
			BallInSocket::new(Anchor::new(Some(a), j.a_pos), Anchor::new(Some(b), j.b_pos))
		}).collect();
		
		let mass = components.iter().filter_map(|ch| ch.body.borrow().mass()).sum();
		
		Entity {
			mass: mass,
			components  : components,
			fixed_joints: fixed_joints,
			ball_joints : ball_joints,
		}
	}
	
	/// Removes this entity from a world.
	pub fn remove_world(&self, world: &mut World<f32>) {
		// Remove ball in socket joints
		for j in self.ball_joints.iter() {
			world.remove_ball_in_socket(j);
		}
		
		// Remove fixed joints
		for j in self.fixed_joints.iter() {
			world.remove_fixed(j);
		}
		
		// Finally, remove rigid bodies
		for c in self.components.iter() {
			world.remove_body(&c.body);
		}
	}
	
	/// Returns the mass of the entity.
	pub fn mass(&self) -> f32 {
		self.mass
	}
	
	/// Renders the entity
	pub fn render(&self, r: &mut Render) {
		for c in self.components {
			c.mesh.render(r, c.body.borrow().position().to_homogeneous());
		}
	}
}
