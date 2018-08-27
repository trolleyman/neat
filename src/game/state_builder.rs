use prelude::*;
use std::rc::Rc;

use std::cell::RefCell;
use glium::Texture2d;
use glutin::{Event, MouseScrollDelta, WindowEvent};
use glutin::dpi::LogicalPosition;
use nc::shape::{ShapeHandle, Ball, Cuboid};
use rand;

use super::state::FONT_SIZE;
use game::{EntityBuilder, GameState, Gravity, Component, TickCallback, RenderCallback};
use render::{Render, Camera, SimpleMesh, ColoredMesh, Material, LitMesh, Light, Color};
use settings::Settings;
use vfs;

pub struct GameStateBuilder {}
/// Util functions for creating a GameState
impl GameStateBuilder {
	/// Builds the default GameState. Used in the default main.rs for easy prototyping.
	pub fn build_default(ctx: &Rc<Context>) -> GameState {
		GameStateBuilder::build_tables(ctx)
	}
	
	/// Builds the `solar` scene.
	/// 
	/// This scene consists of 3 balls:
	/// - One is yellow which represents the sun. This has a large weight.
	/// - One is red which represents mercury. This has a tiny weight.
	/// - One is green which represents earth. This has a medium weight.
	/// 
	/// The yellow ball should oscillate around the centre of the scene.
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
		
		let mut state = GameState::new(Camera::new(Vector3::new(0.0, 0.0, 20.0)), Gravity::Relative(1.0));
		let sun     = EntityBuilder::new(DENSITY, 1.0, 0.0)
			.component(Component::new(Ball::new(SUN_RADIUS), yellow))
			.pos(Vector3::new(SUN_POS, 0.0, 0.0))
			.vel(Vector3::new(0.0, 0.0, SUN_VEL))
			.build(&mut state);
		
		let earth   = EntityBuilder::new(DENSITY, 1.0, 0.0)
			.component(Component::new(Ball::new(EARTH_RADIUS), green))
			.pos(Vector3::new(EARTH_POS, 0.0, 0.0))
			.vel(Vector3::new(0.0, 0.0, -EARTH_VEL))
			.build(&mut state);
		
		let mercury = EntityBuilder::new(DENSITY, 1.0, 0.0)
			.component(Component::new(Ball::new(MERCURY_RADIUS), red))
			.pos(Vector3::new(MERCURY_POS, 0.0, 0.0))
			.vel(Vector3::new(0.0, 0.0, -MERCURY_VEL))
			.build(&mut state);
		
		info!("SUN    : vel: {:6.2}, scale: {:.4}, mass: {:6.2}, radius: {:.4}",
			SUN_VEL,
			1.0,
			1.0 / state.get_entity_rigid_body(sun).unwrap().augmented_mass().mass(),
			SUN_RADIUS);
		info!("EARTH  : vel: {:6.2}, scale: {:.4}, mass: {:6.2}, radius: {:.4}",
			EARTH_VEL,
			EARTH_SCALE,
			1.0 / state.get_entity_rigid_body(earth).unwrap().augmented_mass().mass(),
			EARTH_RADIUS);
		info!("MERCURY: vel: {:6.2}, scale: {:.4}, mass: {:6.2}, radius: {:.4}",
			MERCURY_VEL,
			MERCURY_SCALE,
			1.0 / state.get_entity_rigid_body(mercury).unwrap().augmented_mass().mass(),
			MERCURY_RADIUS);
		
		state
	}
	
	/// Builds the `rot_test` scene.
	/// 
	/// This scene consists of a set of cubes that rotate around a different axis at different speeds.
	pub fn build_rot_test(ctx: &Rc<Context>) -> GameState {
		let sphere = Rc::new(SimpleMesh::sphere(ctx, 0));
		
		let red   = Rc::new(ColoredMesh::new(sphere.clone(), Color::RED));
		let green = Rc::new(ColoredMesh::new(sphere.clone(), Color::GREEN));
		let blue  = Rc::new(ColoredMesh::new(sphere.clone(), Color::BLUE));
		
		let mut state = GameState::new(Camera::new(Vector3::new(2.0, 0.0, 10.0)), Gravity::None);
		
		EntityBuilder::new(1.0, 0.9, 0.1)
			.component(Component::new(Ball::new(1.0), red.clone()))
			.pos(Vector3::new(0.0, 0.0, 0.0))
			.build(&mut state);
		
		EntityBuilder::new(1.0, 0.9, 0.1)
			.component(Component::new(Ball::new(1.0), green.clone()))
			.pos(Vector3::new(3.0, 0.0, 0.0))
			.ang_vel(Vector3::new(1.0, 0.0, 0.0))
			.build(&mut state);
		
		EntityBuilder::new(1.0, 0.9, 0.1)
			.component(Component::new(Ball::new(1.0), blue.clone()))
			.pos(Vector3::new(6.0, 0.0,  0.0))
			.ang_vel(Vector3::new(2.0, 0.0, 0.0))
			.build(&mut state);
		
		EntityBuilder::new(1.0, 0.9, 0.1)
			.component(Component::new(Ball::new(1.0), green.clone()))
			.pos(Vector3::new(0.0, 3.0, 0.0))
			.ang_vel(Vector3::new(0.0, 1.0, 0.0))
			.build(&mut state);
		
		EntityBuilder::new(1.0, 0.9, 0.1)
			.component(Component::new(Ball::new(1.0), blue.clone()))
			.pos(Vector3::new(0.0, 6.0, 0.0))
			.ang_vel(Vector3::new(0.0, 2.0, 0.0))
			.build(&mut state);
		
		EntityBuilder::new(1.0, 0.9, 0.1)
			.component(Component::new(Ball::new(1.0), green.clone()))
			.pos(Vector3::new(0.0, 0.0, 3.0))
			.ang_vel(Vector3::new(0.0, 0.0, 1.0))
			.build(&mut state);
		
		EntityBuilder::new(1.0, 0.9, 0.1)
			.component(Component::new(Ball::new(1.0), blue.clone()))
			.pos(Vector3::new(0.0, 0.0, 6.0))
			.ang_vel(Vector3::new(0.0, 0.0, 2.0))
			.build(&mut state);
		
		state
	}
	
	/// Builds the `spceballs` scene.
	/// 
	/// This scene consists of 3 balls, red, green, and blue that attract one another.
	/// They all have different initial velocities.
	pub fn build_spaceballs(ctx: &Rc<Context>) -> GameState {
		let sphere = Rc::new(SimpleMesh::sphere(ctx, 4));
		
		let red   = Rc::new(ColoredMesh::new(sphere.clone(), Color::RED));
		let green = Rc::new(ColoredMesh::new(sphere.clone(), Color::GREEN));
		let blue  = Rc::new(ColoredMesh::new(sphere.clone(), Color::BLUE));
		
		let mut state = GameState::new(Camera::new(Vector3::new(2.0, 2.0, 10.0)), Gravity::Relative(1.0));
		EntityBuilder::new(1.0, 0.9, 0.1)
			.component(Component::new(Ball::new(1.0), red))
			.pos(Vector3::new(5.0, 0.0,  0.0))
			.vel(Vector3::new(0.0, 1.0, -1.0))
			.build(&mut state);
		
		EntityBuilder::new(1.0, 0.9, 0.1)
			.component(Component::new(Ball::new(1.0), green))
			.pos(Vector3::new(0.0, 0.0, -5.0))
			.vel(Vector3::new(1.0, -1.0, 1.0))
			.build(&mut state);
		
		EntityBuilder::new(1.0, 0.9, 0.1)
			.component(Component::new(Ball::new(1.0), blue))
			.pos(Vector3::new(0.0, 5.0,  0.0))
			.vel(Vector3::new(-1.0, 1.0, 1.0))
			.build(&mut state);
		
		state
	}
	
	/// Builds the `balls` scene.
	/// 
	/// This scene consists of 4 planes that are rotated inwards to form a simple cone.
	/// There are also balls that are generated at the top of the screen that fall down into the cone.
	/// This shows the physics collision aspect of the system.
	pub fn build_balls(ctx: &Rc<Context>) -> GameState {
		// Gen planes
		let mut state = GameState::new(Camera::new(Vector3::new(0.0, 10.0, 15.0)), Gravity::Constant(Vector3::new(0.0, -9.81, 0.0)));
		
		const ANG: f32 = 0.5;
		
		let he = Vector3::new(20.0, 1.0, 20.0);
		let plane_mesh = Rc::new(SimpleMesh::cuboid(ctx, he));
		let green = Rc::new(ColoredMesh::new(plane_mesh.clone(), Color::GREEN));
		let blue  = Rc::new(ColoredMesh::new(plane_mesh.clone(), Color::BLUE));
		// Plane +X
		EntityBuilder::new_static(0.1, 0.5)
			.component(Component::new(Cuboid::new(he), green.clone()))
			.rot(Rotation3::from_euler_angles(0.0, 0.0, -ANG))
			.build(&mut state);
		// Plane +Z
		EntityBuilder::new_static(0.1, 0.5)
			.component(Component::new(Cuboid::new(he), blue .clone()))
			.rot(Rotation3::from_euler_angles(-ANG, 0.0, 0.0))
			.build(&mut state);
		// Plane -X
		EntityBuilder::new_static(0.1, 0.5)
			.component(Component::new(Cuboid::new(he), green.clone()))
			.rot(Rotation3::from_euler_angles(0.0, 0.0, ANG)).
			build(&mut state);
		// Plane -Y
		EntityBuilder::new_static(0.1, 0.5)
			.component(Component::new(Cuboid::new(he), blue .clone()))
			.rot(Rotation3::from_euler_angles(ANG, 0.0, 0.0)).
			build(&mut state);
		
		// Gen balls at top
		const SCALE: f32 = 0.4;
		let ball = ShapeHandle::new(Ball::new(SCALE));
		let ball_mesh = Rc::new(SimpleMesh::sphere(ctx, 4));
		
		let r = move || { rand::thread_rng().gen::<f32>() };
		
		const N: i32 = 10;
		for x in 0..N {
			let x = (x - N/2) as f32 * 2.0;
			for z in 0..N {
				let z = (z - N/2) as f32 * 2.0;
				let col = Color::new(r(), r(), r());
				let ball_mesh = Rc::new(ColoredMesh::with_scale(ball_mesh.clone(), col, SCALE));
				
				EntityBuilder::new(1.0, 0.3, 0.5)
					.component(Component::with_handle(ball.clone(), ball_mesh))
					.pos(Vector3::new(x, 20.0, z))
					.build(&mut state);
			}
		}
		
		state
	}
	
	/// Builds the `phong` scene.
	/// 
	/// This is basically a lighting test. There are some textured cubes, a textured plane, and a sphere.
	pub fn build_phong(ctx: &Rc<Context>) -> GameState {
		let mut state = GameState::new(Camera::new(Vector3::new(2.0, 2.0, 10.0)), Gravity::None);
		
		let he = Vector3::new(0.5, 0.5, 0.5);
		
		let texture = Rc::new(vfs::load_texture(ctx, "test.png"));
		
		let material = Material::new(
			Vector4::new(0.9, 0.9, 0.9, 1.0),
			Vector4::new(0.9, 0.9, 0.9, 1.0),
			Vector4::new(0.5, 0.5, 0.5, 1.0),
			1.0);
		
		EntityBuilder::new(1.0, 0.9, 0.1)
			.component(Component::new(Cuboid::new(he),
				Rc::new(LitMesh::cuboid(ctx, he, texture.clone(), material.with_scale_rgba(Vector4::new(1.0, 0.0, 0.0, 1.0))))))
			.pos(Vector3::new(5.0, 0.0, 0.0))
			.ang_vel(Vector3::new(1.0, 2.0, 0.0))
			.build(&mut state);
		
		EntityBuilder::new(1.0, 0.9, 0.1)
			.component(Component::new(Cuboid::new(he),
				Rc::new(LitMesh::cuboid(ctx, he, texture.clone(), material.with_scale_rgba(Vector4::new(0.0, 1.0, 0.0, 1.0))))))
				.pos(Vector3::new(0.0, 5.0, 0.0))
				.ang_vel(Vector3::new(2.0, 1.0, 0.0))
				.build(&mut state);
		
		EntityBuilder::new(1.0, 0.9, 0.1)
			.component(Component::new(Cuboid::new(he),
				Rc::new(LitMesh::cuboid(ctx, he, texture.clone(), material.with_scale_rgba(Vector4::new(0.0, 0.0, 1.0, 1.0))))))
				.pos(Vector3::new(0.0, 0.0, 5.0))
				.ang_vel(Vector3::new(0.0, 2.0, 1.0))
				.build(&mut state);
		
		let red = Rc::new(ColoredMesh::with_scale(Rc::new(SimpleMesh::sphere(ctx, 4)), Color::RED, 0.1));
		EntityBuilder::new(1.0, 0.9, 0.1)
			.component(Component::new(Ball::new(0.1), red))
			.build(&mut state);
		
		let sphere_mesh = Rc::new(LitMesh::sphere(ctx, 4, Rc::new(vfs::load_texture(ctx, "white.png")), material));
		
		EntityBuilder::new(1.0, 0.9, 0.1)
			.component(Component::new(Ball::new(1.0), sphere_mesh))
			.pos(Vector3::new(3.0, 2.0, 5.0))
			.build(&mut state);
		
		let he = Vector3::new(20.0, 1.0, 20.0);
		let plane_mesh = Rc::new(LitMesh::cuboid(ctx, he, texture, material));
		EntityBuilder::new(1.0, 0.9, 0.1)
			.component(Component::new(Cuboid::new(he), plane_mesh))
			.pos(Vector3::new(0.0, -3.0, 0.0))
			.build(&mut state);
		
		state.set_ambient_light(Vector4::new(0.1, 0.1, 0.1, 1.0));
		
		state.set_light(Light::new_point_light(
			Vector3::new(0.0, 0.0, 0.0),
			Vector4::new(0.7, 0.7, 0.7, 1.0),
			Vector4::new(0.7, 0.7, 0.7, 1.0),
			1.0, 0.40, 0.22));
		
		let handler = Rc::new(RefCell::new(LightHandler::new()));
		state.set_tick_callback(Some(handler.clone()));
		state.set_render_callback(Some(handler.clone()));
		
		state
	}
	
	/// Builds the `tables` scene.
	/// 
	/// This is basically an entity test scene, testing how entities interact with themselves and other objects.
	pub fn build_tables(ctx: &Rc<Context>) -> GameState {
		fn build_table(ctx: &Rc<Context>, state: &mut GameState, top_tex: Rc<Texture2d>, leg_tex: Rc<Texture2d>, pos: Vector3<f32>, material: Material) {
			let r = move || { rand::thread_rng().gen::<f32>() };
			//let r_neg = move || { rand::thread_rng().gen::<f32>() * 2.0 - 1.0 };
			
			let col = Vector4::new(r(), r(), r(), 1.0);
			let material = material.with_scale_rgba(col);
			
			let table_size = 2.0;
			let top_h = 0.5;
			let leg_h = 1.2;
			let leg_w = 0.4;
			
			let table_size2 = table_size / 2.0;
			let top_h2 = top_h / 2.0;
			let leg_h2 = leg_h / 2.0;
			let leg_w2 = leg_w / 2.0;
			
			let leg_he = Vector3::new(leg_w2, leg_h2, leg_w2);
			let leg_mesh = Rc::new(LitMesh::cuboid(ctx, leg_he, leg_tex, material));
			let leg = Component::new(Cuboid::new(leg_he), leg_mesh);
			
			let top_he = Vector3::new(table_size2, top_h2, table_size2);
			let top_mesh = Rc::new(LitMesh::cuboid(ctx, top_he, top_tex, material));
			let top = Component::new(Cuboid::new(top_he), top_mesh);
			
			let off = table_size2 - leg_w2;
			EntityBuilder::new(1.0, 0.3, 0.6)
				// Add legs
				.component(leg.clone().pos(Vector3::new( off, -top_h2-leg_h2,  off)))
				.component(leg.clone().pos(Vector3::new(-off, -top_h2-leg_h2,  off)))
				.component(leg.clone().pos(Vector3::new( off, -top_h2-leg_h2, -off)))
				.component(leg.clone().pos(Vector3::new(-off, -top_h2-leg_h2, -off)))
				// Add table top
				.component(top)
				.pos(pos)
				.build(state);
		}
		
		let mut state = GameState::new(
			Camera::new(Vector3::new(2.0, 2.0, 10.0)),
			Gravity::Constant(Vector3::new(0.0, -9.81, 0.0)));
		
		let light_pos = Vector3::new(3.0, 3.0, 0.0);
		
		let material = Material::new(
			Vector4::new(0.9, 0.9, 0.9, 1.0),
			Vector4::new(0.9, 0.9, 0.9, 1.0),
			Vector4::new(0.5, 0.5, 0.5, 1.0),
			1.0);
		
		let top_tex = Rc::new(vfs::load_texture(ctx, "test.png"));
		let leg_tex = Rc::new(vfs::load_texture(ctx, "white.png"));
		
		// X- Plane
		let he = Vector3::new(1.0, 20.0, 20.0);
		let mesh = Rc::new(LitMesh::cuboid(ctx, he, top_tex.clone(), material));
		let plane = Component::new(Cuboid::new(he), mesh);
		EntityBuilder::new_static(0.3, 0.7)
			.component(plane.clone().pos(Vector3::new(-20.0, -3.0 + 20.0, 0.0))) // X-
			.component(plane.clone().pos(Vector3::new(20.0, -3.0 + 20.0, 0.0)))  // X+
			.build(&mut state);
		
		// Z- Plane
		let he = Vector3::new(20.0, 20.0, 1.0);
		let mesh = Rc::new(LitMesh::cuboid(ctx, he, top_tex.clone(), material));
		let plane = Component::new(Cuboid::new(he), mesh);
		EntityBuilder::new_static(0.3, 0.7)
			.component(plane.clone().pos(Vector3::new(0.0, -3.0 + 20.0, -20.0))) // Z-
			.component(plane.clone().pos(Vector3::new(0.0, -3.0 + 20.0, 20.0)))  // Z+
			.build(&mut state);
		
		// Y- Plane
		let he = Vector3::new(20.0, 1.0, 20.0);
		let mesh = Rc::new(LitMesh::cuboid(ctx, he, top_tex.clone(), material));
		let plane = Component::new(Cuboid::new(he), mesh);
		EntityBuilder::new_static(0.3, 0.7)
			.component(plane.clone().pos(Vector3::new(0.0, -3.0, 0.0)))        // Y-
			.component(plane.clone().pos(Vector3::new(0.0, -3.0 + 40.0, 0.0))) // Y+
			.build(&mut state);
		
		// Tables
		build_table(ctx, &mut state, top_tex.clone(), leg_tex.clone(), Vector3::new(0.0, 1.0, 0.0)  , material);
		build_table(ctx, &mut state, top_tex.clone(), leg_tex.clone(), Vector3::new(0.3, 3.0, 0.1)  , material);
		build_table(ctx, &mut state, top_tex.clone(), leg_tex.clone(), Vector3::new(-0.5, 6.0, -0.4), material);
		build_table(ctx, &mut state, top_tex.clone(), leg_tex.clone(), Vector3::new(0.5, 9.0, 0.4)  , material);
		
		let f = move |n: usize| { n as f32 * 2.0 - 5.0 };
		for x in 0..10 {
			for z in 0..10 {
				build_table(ctx, &mut state, top_tex.clone(), leg_tex.clone(), Vector3::new(f(x), 12.0, f(z)), material);
			}
		}
		
		// Light indicator
		let red = Rc::new(ColoredMesh::with_scale(Rc::new(SimpleMesh::sphere(ctx, 4)), Color::RED, 0.1));
		EntityBuilder::new_static(0.3, 0.7)
			.component(Component::new(Ball::new(0.1), red))
			.pos(light_pos)
			.build(&mut state);
		
		state.set_ambient_light(Vector4::new(0.0, 0.0, 0.0, 1.0));
		
		state.set_light(Light::new_point_light(
			light_pos,
			Vector4::new(0.7, 0.7, 0.7, 1.0),
			Vector4::new(0.7, 0.7, 0.7, 1.0),
			1.0, 0.40, 0.22));
		
		state
	}
}

#[derive(Debug)]
enum Mode {
	LightConstant,
	LightLinear,
	LightQuadratic,
}
struct LightHandler {
	mode: Mode
}
impl LightHandler {
	pub fn new() -> LightHandler {
		LightHandler {
			mode: Mode::LightConstant
		}
	}
}
impl TickCallback for LightHandler {
	fn tick(&mut self, state: &mut GameState, _dt: f32, _settings: &Settings, events: &[Event], _mouse_moved: Vector2<f64>) {
		const PIXELS_PER_LINE: f32 = 16.0;
		
		let mut scroll: Vector2<f32> = Vector2::zero();
		for event in events.iter() {
			if let Event::WindowEvent{event, ..} = event {
				match event {
					&WindowEvent::MouseWheel{delta: MouseScrollDelta::LineDelta(x, y), ..} => {
						scroll.x += x * PIXELS_PER_LINE;
						scroll.y += y * PIXELS_PER_LINE;
					},
					&WindowEvent::MouseWheel{delta: MouseScrollDelta::PixelDelta(LogicalPosition{x, y}), ..} => {
						scroll.x += x as f32;
						scroll.y += y as f32;
					},
					&WindowEvent::ReceivedCharacter(c) => {
						match match c {
							'1' => Some(Mode::LightConstant),
							'2' => Some(Mode::LightLinear),
							'3' => Some(Mode::LightQuadratic),
							_ => None,
						} {
							Some(m) => {
								info!("Changing mode to {:?}", m);
								self.mode = m;
							},
							None => {}
						}
					}
					_ => {}
				}
			}
		}
		scroll.y *= 0.07;
		
		let mut light = *state.light();
		match self.mode {
			Mode::LightConstant  => light.constant_attenuation  += scroll.y,
			Mode::LightLinear    => light.linear_attenuation    += scroll.y,
			Mode::LightQuadratic => light.quadratic_attenuation += scroll.y,
		}
		state.set_light(light);
	}
}
impl RenderCallback for LightHandler {
	fn render(&mut self, r: &mut Render, _fps: u32) {
		let s = match self.mode {
			Mode::LightConstant  => "constant attenuation",
			Mode::LightLinear    => "linear attenuation",
			Mode::LightQuadratic => "quadratic attenuation",
		};
		r.draw_str(s, 10.0, 20.0 + FONT_SIZE, FONT_SIZE);
	}
}
