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

/// Holds a SimpleMesh and gives it a color and scale so that it can be rendered to the screen.
pub struct ColoredMesh {
	/// The mesh used.
	mesh: Rc<SimpleMesh>,
	/// The color to render the mesh in.
	color: Color,
	/// The scale to render the mesh in.
	scale: f32,
}
impl ColoredMesh {
	/// Constructs a new ColoredMesh with a color. The scale will be 1.0.
	pub fn new(mesh: Rc<SimpleMesh>, color: Color) -> ColoredMesh {
		ColoredMesh::with_scale(mesh, color, 1.0)
	}
	
	/// Constructs a new ColoredMesh with a color and a scale.
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

/// A mesh with no vertices that can be rendered.
///
/// Rendering is a no-op.
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
