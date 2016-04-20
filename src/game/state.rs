use std::rc::Rc;
use std::mem;
use std::collections::HashMap;

use glium::backend::Context;
use na::{Norm, Vec3};
use nc::shape::Ball;
use np::world::World;

use game::{KeyboardState, Entity, GameState};
use render::{Camera, Render, SimpleMesh, ColoredMesh, Color};
use settings::Settings;

const FONT_SIZE: f32 = 24.0;

/// Gravity type of the simulation
#[derive(Copy, Clone)]
pub enum Gravity {
	/// Each object attracts each other object
	Relative,
	/// Each object is attracted in a constant direction
	Constant(Vec3<f32>),
}

pub struct State {
	camera: Camera,
	gravity: Gravity,
	world: World<f32>,
}
impl State {
	pub fn new(cam: Camera, g: Gravity) -> State {
		State {
			camera: cam,
			gravity: g,
			world: World::new(),
		}
	}
	
	pub fn gen_balls(ctx: &Rc<Context>, cam: Camera) -> State {
		// TODO: RigidBody builder to sort out all of this mess
		
		let mut state = GameState::new(cam, Gravity::Relative);
		state
	}
	
	pub fn gen_solar(ctx: &Rc<Context>, cam: Camera) -> State {

		let mut state = GameState::new(cam, Gravity::Relative);
		state
	}
	
	pub fn camera(&self) -> &Camera {
		&self.camera
	}
	
	pub fn tick(&mut self, dt: f32, settings: &Settings, keyboard: &KeyboardState, mouse_state: (i32, i32)) {
		// m/s
		let speed = 4.0 * dt;
		
		// Translate camera based on keyboard state
		let mut trans = Vec3::new(0.0, 0.0, 0.0);
		if keyboard.is_pressed(&settings.forward) {
			trans = trans + Vec3::new(0.0, 0.0, -speed);
		}
		if keyboard.is_pressed(&settings.backward) {
			trans = trans + Vec3::new(0.0, 0.0,  speed);
		}
		if keyboard.is_pressed(&settings.left) {
			trans = trans + Vec3::new(-speed, 0.0, 0.0);
		}
		if keyboard.is_pressed(&settings.right) {
			trans = trans + Vec3::new( speed, 0.0, 0.0);
		}
		if keyboard.is_pressed(&settings.up) {
			trans = trans + Vec3::new(0.0,  speed, 0.0);
		}
		if keyboard.is_pressed(&settings.down) {
			trans = trans + Vec3::new(0.0, -speed, 0.0);
		}
		self.camera.translate(trans);
		self.camera.mouse_moved(mouse_state.0, mouse_state.1);
		
		if !settings.paused {
			
			// Tick world
			self.world.step(dt);
		}
	}

	pub fn render(&mut self, r: &mut Render, fps: u32) {
		r.set_camera(self.camera);
		
		r.draw_str(&format!("{} FPS", fps), 10.0, 10.0, FONT_SIZE);
		
		r.swap();
	}
}
