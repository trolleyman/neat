use prelude::*;
use std::rc::Rc;

use super::{Color, Render};
use util;

pub use self::simple::{SimpleVertex, SimpleMesh};
pub use self::lit::{LitVertex, LitMesh};

mod simple;
mod lit;

/// Represents a mesh that can be rendered.
pub trait RenderableMesh {
	fn render(&self, r: &mut Render, model: Mat4<f32>);
}

pub struct ColoredMesh {
	mesh: Rc<SimpleMesh>,
	color: Color,
	scale: f32,
}
impl ColoredMesh {
	pub fn new(mesh: Rc<SimpleMesh>, color: Color) -> ColoredMesh {
		ColoredMesh::with_scale(mesh, color, 1.0)
	}
	
	pub fn with_scale(mesh: Rc<SimpleMesh>, color: Color, scale: f32) -> ColoredMesh {
		ColoredMesh {
			mesh : mesh,
			color: color,
			scale: scale,
		}
	}
}
impl RenderableMesh for ColoredMesh {
	fn render(&self, r: &mut Render, model: Mat4<f32>) {
		let scale = util::mat4_scale(Vec3::new(self.scale, self.scale, self.scale));
		self.mesh.render(r, model * scale, self.color);
	}
}

pub struct EmptyMesh {
	
}
impl EmptyMesh {
	pub fn new() -> EmptyMesh {
		EmptyMesh {}
	}
}
impl RenderableMesh for EmptyMesh {
	fn render(&self, _r: &mut Render, _model: Mat4<f32>) {
		// No-op.
	}
}
