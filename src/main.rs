#![feature(box_syntax, question_mark, associated_consts, as_unsafe_cell)]
#[macro_use]
extern crate glium;
extern crate nalgebra as na;
extern crate ncollide as nc;
extern crate nphysics3d as np;
extern crate rusttype;

#[macro_use]
extern crate log;
extern crate simplelog;
extern crate unicode_normalization;
#[macro_use]
extern crate cfg_if;

#[cfg(windows)]
extern crate user32;

use std::io::{self, Write};

pub use glium::glutin;
use na::Vec3;
use simplelog::{TermLogger, LogLevelFilter};

pub mod render;
pub mod game;
pub mod util;
pub mod settings;
pub mod vfs;

use render::Camera;
use game::Game;
use settings::Settings;

pub fn main() {
	
	let mut g = Game::new(Camera::new(Vec3::new(2.0, 2.0, 10.0)));
}
