use prelude::*;
use std::rc::Rc;

use glium::Texture2d;
use nc::shape::{Ball, Cuboid};
use np::object::RigidBody;
use rand;

use game::{EntityBuilder, GameState, Gravity, Component};
use render::{Camera, SimpleMesh, ColoredMesh, Material, LitMesh, Light, Color};
use vfs;

pub struct GameStateBuilder {}
impl GameStateBuilder {
	pub fn build_default(ctx: &Rc<Context>) -> GameState {
		GameStateBuilder::build_tables(ctx)
	}
	
	pub fn build_spaceballs(ctx: &Rc<Context>) -> GameState {
		let sphere = Rc::new(SimpleMesh::sphere(ctx, 4));
		
		let red   = Rc::new(ColoredMesh::new(sphere.clone(), Color::RED));
		let green = Rc::new(ColoredMesh::new(sphere.clone(), Color::GREEN));
		let blue  = Rc::new(ColoredMesh::new(sphere.clone(), Color::BLUE));
		
		let mut state = GameState::new(Camera::new(Vec3::new(2.0, 2.0, 10.0)), Gravity::Relative(1.0));
		EntityBuilder::new(Component::new(
			RigidBody::new_dynamic(Ball::new(1.0), 1.0, 0.9, 0.1), red))
				.pos(Vec3::new(5.0, 0.0,  0.0))
				.vel(Vec3::new(0.0, 1.0, -1.0))
				.build(&mut state);
		
		EntityBuilder::new(Component::new(
			RigidBody::new_dynamic(Ball::new(1.0), 1.0, 0.9, 0.1), green))
				.pos(Vec3::new(0.0, 0.0, -5.0))
				.vel(Vec3::new(1.0, -1.0, 1.0))
				.build(&mut state);
		
		EntityBuilder::new(Component::new(
			RigidBody::new_dynamic(Ball::new(1.0), 1.0, 0.9, 0.1), blue))
				.pos(Vec3::new(0.0, 5.0,  0.0))
				.vel(Vec3::new(-1.0, 1.0, 1.0))
				.build(&mut state);
		state
	}
	
	#[allow(non_snake_case)]
	pub fn build_solar(ctx: &Rc<Context>) -> GameState {
		let sphere = Rc::new(SimpleMesh::sphere(ctx, 4));
		
		const PI: f32 = ::std::f32::consts::PI;
		
		const SUN_POS: f32 = 0.0;
		const SUN_MASS: f32 = 100.0;
		const SUN_RADIUS: f32 = 1.0;
		let SUN_VOLUME: f32 = (4.0 * PI * SUN_RADIUS * SUN_RADIUS * SUN_RADIUS) / 3.0;
		let DENSITY: f32 = SUN_MASS / SUN_VOLUME;
		
		const EARTH_POS: f32 = 18.0;
		const EARTH_VEL: f32 = 22.0;
		const EARTH_SCALE: f32 = 0.05;
		let EARTH_RADIUS: f32 = ((3.0 * EARTH_SCALE) / (4.0 * PI)).cbrt();
		
		const MERCURY_POS: f32 = 10.0;
		const MERCURY_VEL: f32 = 30.0;
		const MERCURY_SCALE: f32 = 0.0005;
		let MERCURY_RADIUS: f32 = ((3.0 * MERCURY_SCALE) / (4.0 * PI)).cbrt();
		
		// Equalize forces
		const SUN_VEL: f32 = 0.38;
		
		let yellow = Rc::new(ColoredMesh::with_scale(sphere.clone(), Color::YELLOW, SUN_RADIUS));
		let green  = Rc::new(ColoredMesh::with_scale(sphere.clone(), Color::GREEN , EARTH_RADIUS));
		let red    = Rc::new(ColoredMesh::with_scale(sphere.clone(), Color::RED   , MERCURY_RADIUS));
		
		let mut state = GameState::new(Camera::new(Vec3::new(0.0, 0.0, 20.0)), Gravity::Relative(1.0));
		let sun     = EntityBuilder::new(Component::new(
			RigidBody::new_dynamic(Ball::new(SUN_RADIUS), DENSITY, 1.0, 0.0), yellow))
				.pos(Vec3::new(SUN_POS, 0.0, 0.0))
				.vel(Vec3::new(0.0, 0.0, SUN_VEL))
				.build(&mut state);
		
		let earth   = EntityBuilder::new(Component::new(
			RigidBody::new_dynamic(Ball::new(EARTH_RADIUS), DENSITY, 1.0, 0.0), green))
				.pos(Vec3::new(EARTH_POS, 0.0, 0.0))
				.vel(Vec3::new(0.0, 0.0, -EARTH_VEL))
				.build(&mut state);
		
		let mercury = EntityBuilder::new(Component::new(
			RigidBody::new_dynamic(Ball::new(MERCURY_RADIUS), DENSITY, 1.0, 0.0), red))
				.pos(Vec3::new(MERCURY_POS, 0.0, 0.0))
				.vel(Vec3::new(0.0, 0.0, -MERCURY_VEL))
				.build(&mut state);
		
		info!("SUN    : vel: {:6.2}, scale: {:.4}, mass: {:6.2}, radius: {:.4}",
			SUN_VEL,
			1.0,
			state.get_entity(&sun).unwrap().mass(),
			SUN_RADIUS);
		info!("EARTH  : vel: {:6.2}, scale: {:.4}, mass: {:6.2}, radius: {:.4}",
			EARTH_VEL,
			EARTH_SCALE,
			state.get_entity(&earth).unwrap().mass(),
			EARTH_RADIUS);
		info!("MERCURY: vel: {:6.2}, scale: {:.4}, mass: {:6.2}, radius: {:.4}",
			MERCURY_VEL,
			MERCURY_SCALE,
			state.get_entity(&mercury).unwrap().mass(),
			MERCURY_RADIUS);
		
		state
	}
	
	pub fn build_rot_test(ctx: &Rc<Context>) -> GameState {
		let sphere = Rc::new(SimpleMesh::sphere(ctx, 0));
		
		let red   = Rc::new(ColoredMesh::new(sphere.clone(), Color::RED));
		let green = Rc::new(ColoredMesh::new(sphere.clone(), Color::GREEN));
		let blue  = Rc::new(ColoredMesh::new(sphere.clone(), Color::BLUE));
		
		let mut state = GameState::new(Camera::new(Vec3::new(2.0, 0.0, 10.0)), Gravity::None);
		
		EntityBuilder::new(Component::new(
			RigidBody::new_dynamic(Ball::new(1.0), 1.0, 0.9, 0.1), red.clone()))
				.pos(Vec3::new(0.0, 0.0,  0.0))
				.rot(Vec3::new(0.0, 0.0, 0.0))
				.build(&mut state);
		
		EntityBuilder::new(Component::new(
			RigidBody::new_dynamic(Ball::new(1.0), 1.0, 0.9, 0.1), green.clone()))
				.pos(Vec3::new(3.0, 0.0, 0.0))
				.rot(Vec3::new(0.0, 0.0, 0.0))
				.ang_vel(Vec3::new(1.0, 0.0, 0.0))
				.build(&mut state);
		
		EntityBuilder::new(Component::new(
			RigidBody::new_dynamic(Ball::new(1.0), 1.0, 0.9, 0.1), blue.clone()))
				.pos(Vec3::new(6.0, 0.0,  0.0))
				.rot(Vec3::new(0.0, 0.0, 0.0))
				.ang_vel(Vec3::new(2.0, 0.0, 0.0))
				.build(&mut state);
		
		EntityBuilder::new(Component::new(
			RigidBody::new_dynamic(Ball::new(1.0), 1.0, 0.9, 0.1), green.clone()))
				.pos(Vec3::new(0.0, 3.0, 0.0))
				.rot(Vec3::new(0.0, 0.0, 0.0))
				.ang_vel(Vec3::new(0.0, 1.0, 0.0))
				.build(&mut state);
		
		EntityBuilder::new(Component::new(
			RigidBody::new_dynamic(Ball::new(1.0), 1.0, 0.9, 0.1), blue.clone()))
				.pos(Vec3::new(0.0, 6.0,  0.0))
				.rot(Vec3::new(0.0, 0.0, 0.0))
				.ang_vel(Vec3::new(0.0, 2.0, 0.0))
				.build(&mut state);
		
		EntityBuilder::new(Component::new(
			RigidBody::new_dynamic(Ball::new(1.0), 1.0, 0.9, 0.1), green.clone()))
				.pos(Vec3::new(0.0, 0.0, 3.0))
				.rot(Vec3::new(0.0, 0.0, 0.0))
				.ang_vel(Vec3::new(0.0, 0.0, 1.0))
				.build(&mut state);
		
		EntityBuilder::new(Component::new(
			RigidBody::new_dynamic(Ball::new(1.0), 1.0, 0.9, 0.1), blue.clone()))
				.pos(Vec3::new(0.0, 0.0,  6.0))
				.rot(Vec3::new(0.0, 0.0, 0.0))
				.ang_vel(Vec3::new(0.0, 0.0, 2.0))
				.build(&mut state);
		
		state
	}
	
	pub fn build_balls(ctx: &Rc<Context>) -> GameState {
		// Gen planes
		let mut state = GameState::new(Camera::new(Vec3::new(0.0, 10.0, 15.0)), Gravity::Constant(Vec3::new(0.0, -9.81, 0.0)));
		
		const ANG: f32 = 0.5;
		
		let he = Vec3::new(20.0, 1.0, 20.0);
		let plane_mesh = Rc::new(SimpleMesh::cuboid(ctx, he));
		let green = Rc::new(ColoredMesh::new(plane_mesh.clone(), Color::GREEN));
		let blue  = Rc::new(ColoredMesh::new(plane_mesh.clone(), Color::BLUE));
		let plane_body = RigidBody::new_static(Cuboid::new(he), 0.1, 0.5);
		// Plane +X
		EntityBuilder::new(Component::new(plane_body.clone(), green.clone()))
			.rot(Vec3::new(0.0, 0.0, -ANG)).build(&mut state);
		// Plane +Z
		EntityBuilder::new(Component::new(plane_body.clone(), blue .clone()))
			.rot(Vec3::new(-ANG, 0.0, 0.0)).build(&mut state);
		// Plane -X
		EntityBuilder::new(Component::new(plane_body.clone(), green.clone()))
			.rot(Vec3::new(0.0, 0.0, ANG)).build(&mut state);
		// Plane -Y
		EntityBuilder::new(Component::new(plane_body.clone(), blue .clone()))
			.rot(Vec3::new(ANG, 0.0, 0.0)).build(&mut state);
		
		// Gen balls at top
		const SCALE: f32 = 0.4;
		let ball_body = RigidBody::new_dynamic(Ball::new(SCALE), 1.0, 0.3, 0.5);
		let ball_mesh = Rc::new(SimpleMesh::sphere(ctx, 4));
		const N: i32 = 10;
		for x in 0..N {
			let x = (x - N/2) as f32 * 2.0;
			for z in 0..N {
				let z = (z - N/2) as f32 * 2.0;
				let col = Color::new(1.0, 0.0, 0.0);
				EntityBuilder::new(Component::new(ball_body.clone(), Rc::new(ColoredMesh::with_scale(ball_mesh.clone(), col, SCALE)))).pos(Vec3::new(x, 20.0, z)).build(&mut state);
			}
		}
		
		state
	}
	
	pub fn build_phong(ctx: &Rc<Context>) -> GameState {
		let mut state = GameState::new(Camera::new(Vec3::new(2.0, 2.0, 10.0)), Gravity::None);
		
		let he = Vec3::new(0.5, 0.5, 0.5);
		
		let texture = Rc::new(vfs::load_texture(ctx, "test.png"));
		
		let material = Material::new(
			Vec4::new(0.9, 0.9, 0.9, 1.0),
			Vec4::new(0.9, 0.9, 0.9, 1.0),
			Vec4::new(0.5, 0.5, 0.5, 1.0),
			1.0);
		
		EntityBuilder::new(Component::new(
			RigidBody::new_dynamic(Cuboid::new(he), 1.0, 0.9, 0.1),
			Rc::new(LitMesh::cuboid(ctx, he, texture.clone(), material.with_scale_rgba(Vec4::new(1.0, 0.0, 0.0, 1.0))))))
				.pos(Vec3::new(5.0, 0.0, 0.0))
				.ang_vel(Vec3::new(1.0, 2.0, 0.0))
				.build(&mut state);
		
		EntityBuilder::new(Component::new(
			RigidBody::new_dynamic(Cuboid::new(he), 1.0, 0.9, 0.1),
			Rc::new(LitMesh::cuboid(ctx, he, texture.clone(), material.with_scale_rgba(Vec4::new(0.0, 1.0, 0.0, 1.0))))))
				.pos(Vec3::new(0.0, 5.0, 0.0))
				.ang_vel(Vec3::new(2.0, 1.0, 0.0))
				.build(&mut state);
		
		EntityBuilder::new(Component::new(
			RigidBody::new_dynamic(Cuboid::new(he), 1.0, 0.9, 0.1),
			Rc::new(LitMesh::cuboid(ctx, he, texture.clone(), material.with_scale_rgba(Vec4::new(0.0, 0.0, 1.0, 1.0))))))
				.pos(Vec3::new(0.0, 0.0, 5.0))
				.ang_vel(Vec3::new(0.0, 2.0, 1.0))
				.build(&mut state);
		
		let red = Rc::new(ColoredMesh::with_scale(Rc::new(SimpleMesh::sphere(ctx, 4)), Color::RED, 0.1));
		EntityBuilder::new(Component::new(
			RigidBody::new_dynamic(Ball::new(0.1), 1.0, 0.9, 0.1), red))
				.build(&mut state);
		
		let sphere_mesh = Rc::new(LitMesh::sphere(ctx, 4, Rc::new(vfs::load_texture(ctx, "white.png")), material));
		
		EntityBuilder::new(Component::new(
			RigidBody::new_dynamic(Ball::new(1.0), 1.0, 0.9, 0.1), sphere_mesh.clone()))
				.pos(Vec3::new(3.0, 2.0, 5.0))
				.build(&mut state);
		
		let he = Vec3::new(20.0, 1.0, 20.0);
		let plane_mesh = Rc::new(LitMesh::cuboid(ctx, he, texture, material));
		EntityBuilder::new(Component::new(
			RigidBody::new_dynamic(Cuboid::new(he), 1.0, 0.9, 0.1), plane_mesh))
				.pos(Vec3::new(0.0, -3.0, 0.0))
				.build(&mut state);
		
		state.set_light(Light::new(
			Vec3::new(0.0, 0.0, 0.0),
			Vec4::new(0.0, 0.0, 0.0, 1.0),
			Vec4::new(0.2, 0.2, 0.2, 1.0),
			Vec4::new(0.7, 0.7, 0.7, 1.0)));
		
		state
	}
	
	pub fn build_tables(ctx: &Rc<Context>) -> GameState {
		fn build_table(ctx: &Rc<Context>, state: &mut GameState, top_tex: Rc<Texture2d>, leg_tex: Rc<Texture2d>, pos: Vec3<f32>, material: Material) {
			let r = move || { rand::thread_rng().next_f32() };
			//let r_neg = move || { rand::thread_rng().next_f32() * 2.0 - 1.0 };
			
			let col = Vec4::new(r(), r(), r(), 1.0);
			
			let table_size = 2.0;
			let top_h = 0.5;
			let leg_h = 1.2;
			let leg_w = 0.4;
			
			let table_size2 = table_size / 2.0;
			let top_h2 = top_h / 2.0;
			let leg_h2 = leg_h / 2.0;
			let leg_w2 = leg_w / 2.0;
			
			let leg = Component::new_cuboid(ctx, Vec3::new(leg_w2, leg_h2, leg_w2), 1.0, 0.9, 0.1, leg_tex, material.with_scale_rgba(col));
			
			let mut builder = EntityBuilder::new(
				Component::new_cuboid(ctx, Vec3::new(table_size2, top_h2, table_size2), 1.0, 0.9, 0.1, top_tex, material.with_scale_rgba(col)))
					.pos(pos);
			
			let id = Vec3::x();
			let off = table_size2 - leg_w2;
			builder.add_fixed(0, Iso3::new(Vec3::new( off, -top_h2,  off), id), leg.clone(), Iso3::new(Vec3::y() * leg_h2, id));
			builder.add_fixed(0, Iso3::new(Vec3::new(-off, -top_h2,  off), id), leg.clone(), Iso3::new(Vec3::y() * leg_h2, id));
			builder.add_fixed(0, Iso3::new(Vec3::new( off, -top_h2, -off), id), leg.clone(), Iso3::new(Vec3::y() * leg_h2, id));
			builder.add_fixed(0, Iso3::new(Vec3::new(-off, -top_h2, -off), id), leg.clone(), Iso3::new(Vec3::y() * leg_h2, id));
			
			builder.build(state);
		}
		let mut state = GameState::new(Camera::new(Vec3::new(2.0, 2.0, 10.0)), Gravity::Constant(Vec3::new(0.0, -9.81, 0.0)));
		
		let light_pos = Vec3::new(3.0, 3.0, 0.0);
		
		let material = Material::new(
			Vec4::new(0.9, 0.9, 0.9, 1.0),
			Vec4::new(0.9, 0.9, 0.9, 1.0),
			Vec4::new(0.5, 0.5, 0.5, 1.0),
			1.0);
		
		let top_tex = Rc::new(vfs::load_texture(ctx, "test.png"));
		let leg_tex = Rc::new(vfs::load_texture(ctx, "white.png"));
		
		// Table
		build_table(ctx, &mut state, top_tex.clone(), leg_tex, Vec3::new(0.0, 1.0, 0.0), material);
		
		// Plane
		let he = Vec3::new(20.0, 1.0, 20.0);
		EntityBuilder::new(Component::new_static_cuboid(ctx, he, 0.9, 0.1, top_tex, material))
			.pos(Vec3::new(0.0, -3.0, 0.0))
			.build(&mut state);
		
		// Light indicator
		let red = Rc::new(ColoredMesh::with_scale(Rc::new(SimpleMesh::sphere(ctx, 4)), Color::RED, 0.1));
		EntityBuilder::new(Component::new(
			RigidBody::new_static(Ball::new(0.1), 0.9, 0.1), red))
				.pos(light_pos)
				.build(&mut state);
		
		state.set_light(Light::new(
			light_pos,
			Vec4::new(0.1, 0.1, 0.1, 1.0),
			Vec4::new(0.3, 0.3, 0.3, 1.0),
			Vec4::new(0.7, 0.7, 0.7, 1.0)));
		
		state
	}
}
