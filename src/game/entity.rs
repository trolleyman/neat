use prelude::*;
use std::rc::Rc;
use std::sync::Arc;

use nc::shape::Compound;
use nc::inspection::Repr;
use np::object::{RigidBody, RigidBodyHandle};
use np::world::World;

use game::{GameState, EntityId};
use render::{Render, RenderableMesh};

/// A component of an Entity
#[derive(Clone)]
pub struct Component {
	iso: Iso3<f32>,
	shape: Arc<Box<Repr<Pnt3<f32>, Iso3<f32>> + 'static>>,
	mesh: Rc<RenderableMesh>,
}
impl Component {
	/// Constructs a new component from a shape and a mesh. The position will be at 0,0,0
	pub fn new<S>(shape: S, mesh: Rc<RenderableMesh>) -> Component
			where S: Repr<Pnt3<f32>, Iso3<f32>> {
		Component {
			iso : Iso3::one(),
			shape: Arc::new(box shape as Box<Repr<Pnt3<f32>, Iso3<f32>>>),
			mesh: mesh,
		}
	}
	
	/// Constructs a new component with the default translation.
	pub fn with_arc(shape: Arc<Box<Repr<Pnt3<f32>, Iso3<f32>> + 'static>>, mesh: Rc<RenderableMesh>) -> Component {
		Component {
			iso : Iso3::one(),
			shape: shape,
			mesh: mesh,
		}
	}
	
	/// Constructs a new component from a position, a shape and a mesh.
	pub fn with_iso<S>(iso: Iso3<f32>, shape: S, mesh: Rc<RenderableMesh>) -> Component
			where S: Repr<Pnt3<f32>, Iso3<f32>> {
		Component {
			iso : iso,
			shape: Arc::new(box shape as Box<Repr<Pnt3<f32>, Iso3<f32>>>),
			mesh: mesh,
		}
	}
	
	pub fn with_iso_arc(iso: Iso3<f32>, shape: Arc<Box<Repr<Pnt3<f32>, Iso3<f32>> + 'static>>, mesh: Rc<RenderableMesh>) -> Component {
		Component {
			iso : iso,
			shape: shape,
			mesh: mesh,
		}
	}
	
	/// Returns a component with the specified translation
	pub fn with_pos(mut self, pos: Vec3<f32>) -> Component {
		self.iso.translation = pos;
		self
	}
}

/// Helper struct to build an entity.
pub struct EntityBuilder {
	pos: Vec3<f32>,
	vel: Vec3<f32>,
	rot: Vec3<f32>,
	ang_vel: Vec3<f32>,
	
	// If None, is a static object
	density: Option<f32>,
	restitution: f32,
	friction: f32,
	
	components: Vec<Component>,
}
impl EntityBuilder {
	/// Creates a new dynamic EntityBuilder.
	pub fn new(density: f32, restitution: f32, friction: f32) -> EntityBuilder {
		EntityBuilder {
			pos: Vec3::zero(),
			vel: Vec3::zero(),
			rot: Vec3::zero(),
			ang_vel: Vec3::zero(),
			
			density: Some(density),
			restitution: restitution,
			friction: friction,
			
			components: vec![],
		}
	}
	
	/// Creates a new static EntityBuilder
	pub fn new_static(restitution: f32, friction: f32) -> EntityBuilder {
		EntityBuilder {
			pos: Vec3::zero(),
			vel: Vec3::zero(),
			rot: Vec3::zero(),
			ang_vel: Vec3::zero(),
			
			density: None,
			restitution: restitution,
			friction: friction,
			
			components: vec![],
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
	
	/// Adds a component to the entity.
	pub fn component(mut self, component: Component) -> EntityBuilder {
		self.components.push(component);
		self
	}
	
	/// Builds the entity by adding it to a GameState.
	/// Returns the new entity ID.
	pub fn build(self, state: &mut GameState) -> EntityId {
		state.add_entity(self)
	}
	
	/// Builds the entity by adding it to the world.
	pub fn build_world(self, world: &mut World<f32>) -> Entity {
		Entity::with_matrix(world, self.components, self.pos, self.vel, self.rot, self.ang_vel, self.density, self.restitution, self.friction)
	}
}

pub struct Entity {
	meshes: Vec<(Iso3<f32>, Rc<RenderableMesh>)>,
	body: RigidBodyHandle<f32>,
}
impl Entity {
	pub fn new(world: &mut World<f32>, component: Component, density: Option<f32>, restitution: f32, friction: f32) -> Entity {
		Entity::with_matrix(world, vec![component], Vec3::zero(), Vec3::zero(), Vec3::zero(), Vec3::zero(), density, restitution, friction)
	}
	
	pub fn with_matrix(world: &mut World<f32>, mut components: Vec<Component>, pos: Vec3<f32>, vel: Vec3<f32>, rot: Vec3<f32>, ang_vel: Vec3<f32>, density: Option<f32>, restitution: f32, friction: f32) -> Entity {
		
		let mut bodies = Vec::new();
		let mut meshes = Vec::new();
		for c in components.drain(..) {
			meshes.push((c.iso, c.mesh));
			bodies.push((c.iso, c.shape));
		}
		
		let comp = Compound::new(bodies);
		let body = if let Some(density) = density {
			// Dynamic
			RigidBody::new_dynamic(comp, density, restitution, friction)
		} else {
			// Static
			RigidBody::new_static(comp, restitution, friction)
		};
		let body = world.add_body(body);
		
		let mut e = Entity {
			meshes: meshes,
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
		world.remove_body(&self.body);
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
	
	pub fn set_pos(&mut self, pos: Vec3<f32>) {
		self.body.borrow_mut().set_translation(pos);
	}
	
	pub fn set_vel(&mut self, vel: Vec3<f32>) {
		self.body.borrow_mut().set_lin_vel(vel)
	}
	
	pub fn set_rot(&mut self, rot: Vec3<f32>) {
		self.body.borrow_mut().set_rotation(rot);
	}
	
	pub fn set_ang_vel(&mut self, ang_vel: Vec3<f32>) {
		self.body.borrow_mut().set_ang_vel(ang_vel);
	}
}
