use prelude::*;
use std::rc::Rc;
use std::sync::Arc;

use nc::bounding_volume::{HasBoundingVolume, AABB};
use nc::shape::{ShapeHandle3, ShapeHandle, Cuboid, Compound3, Compound};
use nc::inspection::Repr;
use np::object::{RigidBody, RigidBodyHandle};
use np::world::World;
use np::volumetric::Volumetric;

use game::{GameState, EntityId};
use render::{Render, RenderableMesh};

/// Collision type of an entity.
pub enum Collision {
	Box,
	Compound,
}

/// A component of an entity
#[derive(Clone)]
pub struct Component {
	iso: Isometry3<f32>,
	shape: ShapeHandle3<f32>,
	mesh: Rc<RenderableMesh>,
}
impl Component {
	/// Constructs a new component from a shape and a mesh. The position will be at 0,0,0
	pub fn new<S>(shape: S, mesh: Rc<RenderableMesh>) -> Component
			where S: Repr<Point3<f32>, Isometry3<f32>> {
		Component {
			iso : Isometry3::one(),
			shape: ShapeHandle::new(shape),
			mesh: mesh,
		}
	}
	
	/// Constructs a new component with the default translation.
	pub fn with_handle(shape: ShapeHandle3<f32>, mesh: Rc<RenderableMesh>) -> Component {
		Component {
			iso : Isometry3::one(),
			shape: shape,
			mesh: mesh,
		}
	}
	
	/// Constructs a new component from a position, a shape and a mesh.
	pub fn with_iso<S>(iso: Isometry3<f32>, shape: S, mesh: Rc<RenderableMesh>) -> Component
			where S: Repr<Point3<f32>, Isometry3<f32>> {
		Component {
			iso : iso,
			shape: ShapeHandle::new(shape),
			mesh: mesh,
		}
	}
	
	pub fn with_iso_handle(iso: Isometry3<f32>, shape: ShapeHandle3<f32>, mesh: Rc<RenderableMesh>) -> Component {
		Component {
			iso : iso,
			shape: shape,
			mesh: mesh,
		}
	}
	
	/// Returns the component with the specified translation
	pub fn pos(mut self, pos: Vector3<f32>) -> Component {
		self.iso.translation = pos;
		self
	}
	
	/// Returns the component with the specified rotation
	pub fn rot(mut self, rot: Rotation3<f32>) -> Component {
		self.iso.rotation = rot;
		self
	}
}

/// Helper struct to build an entity.
pub struct EntityBuilder {
	pos: Vector3<f32>,
	vel: Vector3<f32>,
	rot: Vector3<f32>,
	ang_vel: Vector3<f32>,
	
	// If None, is a static object
	density: Option<f32>,
	restitution: f32,
	friction: f32,
	
	collision: Collision,
	components: Vec<Component>,
}
impl EntityBuilder {
	/// Creates a new dynamic EntityBuilder.
	pub fn new(density: f32, restitution: f32, friction: f32) -> EntityBuilder {
		EntityBuilder {
			pos: Vector3::zero(),
			vel: Vector3::zero(),
			rot: Vector3::zero(),
			ang_vel: Vector3::zero(),
			
			density: Some(density),
			restitution: restitution,
			friction: friction,
			
			collision: Collision::Compound,
			components: vec![],
		}
	}
	
	/// Creates a new static EntityBuilder
	pub fn new_static(restitution: f32, friction: f32) -> EntityBuilder {
		EntityBuilder {
			pos: Vector3::zero(),
			vel: Vector3::zero(),
			rot: Vector3::zero(),
			ang_vel: Vector3::zero(),
			
			density: None,
			restitution: restitution,
			friction: friction,
			
			collision: Collision::Compound,
			components: vec![],
		}
	}
	
	/// Sets the position that the entity is created at.
	pub fn pos(mut self, pos: Vector3<f32>) -> EntityBuilder {
		self.pos = pos;
		self
	}
	
	/// Sets the velocity that the entity is created with.
	pub fn vel(mut self, vel: Vector3<f32>) -> EntityBuilder {
		self.vel = vel;
		self
	}
	
	/// Sets the rotation the entity is created with.
	pub fn rot(mut self, rot: Vector3<f32>) -> EntityBuilder {
		self.rot = rot;
		self
	}
	
	/// Sets the angular velocity the entity is created with.
	pub fn ang_vel(mut self, ang_vel: Vector3<f32>) -> EntityBuilder {
		self.ang_vel = ang_vel;
		self
	}
	
	/// Adds a component to the entity.
	pub fn component(mut self, component: Component) -> EntityBuilder {
		self.components.push(component);
		self
	}
	
	/// Sets the collision type of the entity. (Default = Collision::Compound).
	pub fn collision(mut self, collision: Collision) -> EntityBuilder {
		self.collision = collision;
		self
	}
	
	/// The entity will have a collision mesh that is the sum of it's parts. This is more
	/// computationally intensive than box collision.
	pub fn compound_collision(mut self) -> EntityBuilder {
		self.collision = Collision::Compound;
		self
	}
	
	/// The entity will have a collision mesh that is a box. This is less computationally
	/// intensive than compound collision.
	pub fn box_collision(mut self) -> EntityBuilder {
		self.collision = Collision::Box;
		self
	}
	
	/// Builds the entity by adding it to a GameState.
	/// Returns the new entity ID.
	pub fn build(self, state: &mut GameState) -> EntityId {
		state.add_entity(self)
	}
	
	/// Builds the entity by adding it to the world.
	pub fn build_world(self, world: &mut World<f32>) -> Entity {
		Entity::with_matrix(world, self.components, self.collision, self.pos, self.vel, self.rot, self.ang_vel, self.density, self.restitution, self.friction)
	}
}

pub struct Entity {
	meshes: Vec<(Isometry3<f32>, Rc<RenderableMesh>)>,
	collision: Collision,
	body: RigidBodyHandle<f32>,
}
impl Entity {
	pub fn new(world: &mut World<f32>, component: Component, collision: Collision, density: Option<f32>, restitution: f32, friction: f32) -> Entity {
		Entity::with_matrix(world, vec![component], collision: Collision, Vector3::zero(), Vector3::zero(), Vector3::zero(), Vector3::zero(), density, restitution, friction)
	}
	
	pub fn with_matrix(world: &mut World<f32>, mut components: Vec<Component>, collision: Collision, pos: Vector3<f32>, vel: Vector3<f32>, rot: Vector3<f32>, ang_vel: Vector3<f32>, density: Option<f32>, restitution: f32, friction: f32) -> Entity {
		
		let mut bodies = Vec::new();
		let mut meshes = Vec::new();
		for c in components.drain(..) {
			meshes.push((c.iso, c.mesh));
			bodies.push((c.iso, c.shape));
		}
		
		let (collision_shape, mass_props): (ShapeHandle<_, _>, Option<_>) = match collision {
			Collision::Box => {
				let comp: Compound3<f32> = Compound::new(bodies);
				let props = density.map(|density| {
					comp.mass_properties(density)
				});
				let comp_box: AABB<_> = comp.bounding_volume(&Isometry3::one());
				let mins = *comp_box.mins();
				let maxs = *comp_box.maxs();
				let avg  = Vector3::new((mins.x + maxs.x) / 2.0, (mins.y + maxs.y) / 2.0, (mins.z + maxs.z) / 2.0);
				let size = Vector3::new((mins.x - maxs.x) / 2.0, (mins.y - maxs.y) / 2.0, (mins.z - maxs.z) / 2.0);
				let comp_box = Cuboid::new(size);
				let comp_box = Compound::new(vec![(Isometry3::new(-avg, Vector3::zero()), ShapeHandle::new(comp_box))]);
				(ShapeHandle::new(comp_box), props)
			},
			Collision::Compound => {
				let comp: Compound3<f32> = Compound::new(bodies);
				let props = density.map(|density| {
					comp.mass_properties(density)
				});
				(ShapeHandle::new(comp), props)
			}
		};
		
		let mut body = RigidBody::new(collision_shape, mass_props, restitution, friction);
		body.set_translation(pos);
		body.set_rotation(rot);
		let body = world.add_rigid_body(body);
		
		let mut e = Entity {
			meshes: meshes,
			collision: collision,
			body: body,
		};
		
		e.set_pos(pos);
		e.set_vel(vel);
		e.set_rot(rot);
		e.set_ang_vel(ang_vel);
		
		e
	}
	
	/// Removes this entity from a world.
	pub fn remove_world(&self, world: &mut World<f32>) {
		world.remove_rigid_body(&self.body);
	}
	
	/// Renders the entity
	pub fn render(&self, r: &mut Render) {
		let model = self.body.borrow().position().to_homogeneous();
		for &(ref iso, ref mesh) in self.meshes.iter() {
			mesh.render(r, model * iso.to_homogeneous());
		}
	}
	
	/// Gets the RigidBody of the Entity
	pub fn body(&self) -> &RigidBodyHandle<f32> {
		&self.body
	}
	
	pub fn set_pos(&mut self, pos: Vector3<f32>) {
		self.body.borrow_mut().set_translation(pos);
	}
	
	pub fn set_vel(&mut self, vel: Vector3<f32>) {
		self.body.borrow_mut().set_lin_vel(vel)
	}
	
	pub fn set_rot(&mut self, rot: Vector3<f32>) {
		self.body.borrow_mut().set_rotation(rot);
	}
	
	pub fn set_ang_vel(&mut self, ang_vel: Vector3<f32>) {
		self.body.borrow_mut().set_ang_vel(ang_vel);
	}
}
