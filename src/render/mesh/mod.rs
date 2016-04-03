use std::rc::Rc;

use cgmath::Matrix4;

use super::{Color, Render};

pub use self::simple::Mesh as SimpleMesh;
pub use self::simple::Vertex as SimpleVertex;

mod simple;

pub trait RenderableMesh {
	fn render(&self, r: &mut Render, model: Matrix4<f32>);
}

pub struct ColoredMesh {
	mesh: Rc<SimpleMesh>,
	color: Color,
}
impl ColoredMesh {
	pub fn new(mesh: Rc<SimpleMesh>, color: Color) -> ColoredMesh {
		ColoredMesh {
			mesh : mesh,
			color: color,
		}
	}
}
impl RenderableMesh for ColoredMesh {
	fn render(&self, r: &mut Render, model: Matrix4<f32>) {
		self.mesh.render(r, model, self.color);
	}
}
