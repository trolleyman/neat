use prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use nc::shape::Cuboid;
use np::object::{RigidBody, RigidBodyHandle, RigidBodyCollisionGroups};
use np::volumetric::Volumetric;
use np::detection::joint::{Anchor, Fixed, BallInSocket};
use np::world::World;
use glium::Texture2d;

use game::{GameState, EntityId};
use render::{Render, RenderableMesh, Material, LitMesh};

/// ID of the root component in an entity.
pub type ComponentId = u32;
pub const ROOT_ID: ComponentId = 0;

/// A component of an Entity
#[derive(Clone)]
pub struct Component {
	body: RigidBody<f32>,
	mesh: Rc<RenderableMesh>,
}
impl Component {
	/// Constructs  a new component from a rigidbody and a mesh.
	pub fn new(body: RigidBody<f32>, mesh: Rc<RenderableMesh>) -> Component {
		Component {
			body: body,
			mesh: mesh,
		}
	}
	
	/// Helper function to construct a new cuboid from it's half extents.
	pub fn new_cuboid(ctx: &Rc<Context>, half_extents: Vec3<f32>, density: f32, restitution: f32, friction: f32, texture: Rc<Texture2d>, material: Material) -> Component {
		Component::new(
			RigidBody::new_dynamic(Cuboid::new(half_extents), density, restitution, friction),
			Rc::new(LitMesh::cuboid(ctx, half_extents, texture, material)),
		)
	}
	
	/// Helper function to construct a static cuboid from it's half extents.
	pub fn new_static_cuboid(ctx: &Rc<Context>, half_extents: Vec3<f32>, restitution: f32, friction: f32, texture: Rc<Texture2d>, material: Material) -> Component {
		Component::new(
			RigidBody::new_static(Cuboid::new(half_extents), restitution, friction),
			Rc::new(LitMesh::cuboid(ctx, half_extents, texture, material)),
		)
	}
}

/// A handle to a component in the simulation
pub struct ComponentHandle {
	body: RigidBodyHandle<f32>,
	mesh: Rc<RenderableMesh>,
}
impl ComponentHandle {
	pub fn body(&self) -> &RigidBodyHandle<f32> {
		&self.body
	}
	
	pub fn mesh(&self) -> &Rc<RenderableMesh> {
		&self.mesh
	}
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

/// Helper struct to build an entity.
pub struct EntityBuilder {
	pos: Vec3<f32>,
	vel: Vec3<f32>,
	rot: Vec3<f32>,
	ang_vel: Vec3<f32>,
	
	components: Vec<Component>,
	fixed_joints: Vec<FixedIds>,
	ball_joints: Vec<BallInSocketIds>,
}
impl EntityBuilder {
	/// Creates a new EntityBuilder with a root component.
	pub fn new(root: Component) -> EntityBuilder {
		EntityBuilder {
			pos: Vec3::new(0.0, 0.0, 0.0),
			vel: Vec3::new(0.0, 0.0, 0.0),
			rot: Vec3::new(0.0, 0.0, 0.0),
			ang_vel: Vec3::new(0.0, 0.0, 0.0),
			
			components  : vec![root],
			fixed_joints: Vec::new(),
			ball_joints : Vec::new(),
		}
	}
	
	/// Sets the position that the entity is created at.
	pub fn pos(mut self, pos: Vec3<f32>) -> EntityBuilder {
		self.pos = pos;
		self
	}
	
	/// Sets the velocity that the entity is created with.
	pub fn vel(mut self, vel: Vec3<f32>) -> EntityBuilder {
		self.vel = vel;
		self
	}
	
	/// Sets the rotation the entity is created with.
	pub fn rot(mut self, rot: Vec3<f32>) -> EntityBuilder {
		self.rot = rot;
		self
	}
	
	/// Sets the angular velocity the entity is created with.
	pub fn ang_vel(mut self, ang_vel: Vec3<f32>) -> EntityBuilder {
		self.ang_vel = ang_vel;
		self
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
		let len = self.components.len() as ComponentId;
		if a_id >= len {
			panic!("a_id {:?} is not valid.", a_id);
		}
		
		self.components.push(b);
		let b_id = len;
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
		let len = self.components.len() as ComponentId;
		if a_id >= len {
			panic!("a_id {:?} is not valid.", a_id);
		}
		
		self.components.push(b);
		let b_id = len;
		self.fixed_joints.push(FixedIds::new(a_id, a_pos, b_id, b_pos));
		b_id
	}
	
	/// Builds the entity by adding it to a GameState.
	/// Returns the new entity ID.
	pub fn build(self, state: &mut GameState) -> EntityId {
		state.add_entity(self)
	}
	
	/// Builds the entity by adding it to the world.
	pub fn build_world(self, world: &mut World<f32>, collision_group: Option<usize>) -> Entity {
		Entity::with_matrix(world, self.components, self.fixed_joints, self.ball_joints, collision_group, self.pos, self.vel, self.rot, self.ang_vel)
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
		Entity::with_joints(world, vec![component], Vec::new(), Vec::new(), None)
	}
	
	pub fn with_joints(world: &mut World<f32>, components: Vec<Component>, fixed_joints: Vec<FixedIds>, ball_joints: Vec<BallInSocketIds>, collision_group: Option<usize>) -> Entity {
		Entity::with_matrix(world, components, fixed_joints, ball_joints, collision_group, Vec3::zero(), Vec3::zero(), Vec3::zero(), Vec3::zero())
	}
	
	pub fn with_matrix(world: &mut World<f32>, mut components: Vec<Component>, mut fixed_joints: Vec<FixedIds>, mut ball_joints: Vec<BallInSocketIds>, collision_group: Option<usize>, pos: Vec3<f32>, vel: Vec3<f32>, rot: Vec3<f32>, ang_vel: Vec3<f32>) -> Entity {
		enum Joint {
			Fixed(FixedIds),
			BallInSocket(BallInSocketIds),
		}
		// Maps a component id to it's joint.
		let mut joints_map: HashMap<ComponentId, Joint> = HashMap::new();
		for j in fixed_joints.drain(..) {
			assert!(joints_map.insert(j.b, Joint::Fixed(j)).is_none(), "Component has more than one root joint.");
		}
		for j in ball_joints.drain(..) {
			assert!(joints_map.insert(j.b, Joint::BallInSocket(j)).is_none(), "Component has more than one root joint.");
		}
		
		// TODO: Sort out rotation for non-root component
		// Add all the components to the world
		let mut diff = None;
		let components: Vec<_> = components.drain(..).enumerate().map(|(i, mut c)| {
			if let Some(diff) = diff {
				let j = joints_map.get(&(i as ComponentId)).expect("Component in Entity not attached to root.");
				c.body.append_translation(&diff);
				// FIXME: Doesn't work with a component linked to a component linked to the root component.
				match j {
					&Joint::Fixed(ref j) => {
						// TODO: Sort out what happens when the joint has rotated.
						c.body.append_translation(&j.a_pos.translation);
						c.body.append_translation(&-j.b_pos.translation);
					},
					&Joint::BallInSocket(ref j) => {
						c.body.append_translation(&j.a_pos.to_vec());
						c.body.append_translation(&-j.b_pos.to_vec());
					}
				}
			} else {
				let mut root = &mut c.body;
				diff = Some(pos - root.position().translation);
				root.set_translation(pos);
				// Only apply rotation to root, for now.
				root.set_rotation(rot);
				root.set_ang_vel(ang_vel);
			}
			
			// Add to `collision_group`
			if let Some(collision_group) = collision_group {
				let mut groups = RigidBodyCollisionGroups::new_dynamic();
				groups.modify_membership(collision_group, true);
				groups.disable_self_collision();
				c.body.set_collision_groups(groups);
				debug!("Added component to collision group {}", collision_group);
			}
			c.body.set_lin_vel(vel);
			c.body.set_deactivation_threshold(None);
			ComponentHandle {
				body: world.add_body(c.body),
				mesh: c.mesh,
			}
		}).collect();
		
		// Add all the joints to the world
		let mut fixed_joints = Vec::new();
		let mut ball_joints = Vec::new();
		
		for (_, j) in joints_map.drain() {
			match j {
				Joint::Fixed(j) => {
					let a = components[j.a as usize].body.clone();
					let b = components[j.b as usize].body.clone();
					fixed_joints.push(world.add_fixed(Fixed::new(Anchor::new(Some(a), j.a_pos), Anchor::new(Some(b), j.b_pos))));
				},
				Joint::BallInSocket(j) => {
					let a = components[j.a as usize].body.clone();
					let b = components[j.b as usize].body.clone();
					ball_joints.push(world.add_ball_in_socket(BallInSocket::new(Anchor::new(Some(a), j.a_pos), Anchor::new(Some(b), j.b_pos))));
				}
			}
		}
		
		// Get total mass.
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
	
	/// Returns the constituent parts of the entity.
	pub fn components(&self) -> &[ComponentHandle] {
		&self.components
	}
	
	/// Returns the total mass of the entity.
	pub fn mass(&self) -> f32 {
		self.mass
	}
	
	/// Renders the entity
	pub fn render(&self, r: &mut Render) {
		for c in self.components.iter() {
			let model = unsafe {
				(*c.body.as_unsafe_cell().get()).position().to_homogeneous()
			};
			c.mesh.render(r, model);
		}
	}
	
	pub fn set_pos(&mut self, pos: Vec3<f32>) {
		let mut root = self.components[ROOT_ID as usize].body().borrow_mut();
		let diff = pos - root.position().translation;
		root.set_translation(pos);
		
		for i in 1..self.components.len() {
			self.components[i].body().borrow_mut().append_translation(&diff);
		}
	}
	
	pub fn set_vel(&mut self, vel: Vec3<f32>) {
		for c in self.components.iter() {
			c.body().borrow_mut().set_lin_vel(vel)
		}
	}
	
	pub fn set_rot(&mut self, rot: Vec3<f32>) {
		// TODO: Rotate all components around root origin
		self.components[ROOT_ID as usize].body().borrow_mut().set_rotation(rot);
	}
	
	pub fn set_ang_vel(&mut self, ang_vel: Vec3<f32>) {
		// TODO: Calculate correct angular velocity (and real velocity) for each component
		self.components[ROOT_ID as usize].body().borrow_mut().set_ang_vel(ang_vel);
	}
}
