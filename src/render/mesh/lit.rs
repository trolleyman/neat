use std::rc::Rc;
use std::process::exit;
use std::mem;

use na::{Vec2, Vec3, Mat4, Cross, Norm};
use glium::backend::Context;
use glium::index;
use glium::{Texture2d, IndexBuffer, VertexBuffer};

use render::{RenderableMesh, Material, Render};

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
	pos   : [f32; 3],
	normal: [f32; 3],
	uv    : [f32; 2],
}
implement_vertex!(Vertex, pos, normal, uv);

impl Vertex {
	pub fn new(pos: Vec3<f32>, normal: Vec3<f32>, uv: Vec2<f32>) -> Vertex {
		Vertex {
			pos   : unsafe { mem::transmute(pos) },
			normal: unsafe { mem::transmute(normal) },
			uv    : unsafe { mem::transmute(uv) },
		}
	}
}

pub struct Mesh {
	vertex_buffer: VertexBuffer<Vertex>,
	index_buffer : IndexBuffer<u16>,
	texture      : Rc<Texture2d>,
	material     : Material,
}
impl RenderableMesh for Mesh {
	fn render(&self, r: &mut Render, model: Mat4<f32>) {
		r.render_lit(&self.vertex_buffer, &self.index_buffer, model, &*self.texture, &self.material);
	}
}
impl Mesh {
	pub fn cuboid(ctx: &Rc<Context>, half_extents: Vec3<f32>, texture: Rc<Texture2d>, material: Material) -> Mesh {
		let mut vs: Vec<Vertex> = Vec::new();
		let mut is: Vec<u16> = Vec::new();
		
		Mesh::gen_cuboid(&mut vs, &mut is, half_extents);
		Mesh::from_vecs(ctx, vs, is, texture, material)
	}
	
	fn from_vecs(ctx: &Rc<Context>, vs: Vec<Vertex>, is: Vec<u16>, texture: Rc<Texture2d>, material: Material) -> Mesh {
		let vs = match VertexBuffer::immutable(ctx, &vs) {
			Ok(vs) => vs,
			Err(e) => {
				error!("Could not create vertex buffer: {:?}", e);
				exit(1);
			},
		};
		let is = match IndexBuffer ::immutable(ctx, index::PrimitiveType::TrianglesList, &is) {
			Ok(is) => is,
			Err(e) => {
				error!("Could not create index buffer: {:?}", e);
				exit(1);
			},
		};
		
		Mesh {
			vertex_buffer: vs,
			index_buffer : is,
			texture      : texture,
			material     : material,
		}
	}
	
	fn gen_cuboid(vs: &mut Vec<Vertex>, is: &mut Vec<u16>, half_extents: Vec3<f32>) {
		// v0 --- v1 
		// |          <- Looking forward, normal out of the screen.
		// v2     v3  
		fn gen_quad(vs: &mut Vec<Vertex>, is: &mut Vec<u16>, v0: Vec3<f32>, v1: Vec3<f32>, v2: Vec3<f32>, uv_min: Vec2<f32>, uv_max: Vec2<f32>) {
			let i = vs.len() as u16;
			let v02 = v2-v0;
			let v01 = v1-v0;
			let normal = v02.cross(&v01).normalize();
			let v3 = v0 + v01 + v02;
			
			vs.push(Vertex::new(v0, normal, uv_min));
			vs.push(Vertex::new(v1, normal, Vec2::new(uv_max.x, uv_min.y)));
			vs.push(Vertex::new(v2, normal, Vec2::new(uv_min.x, uv_max.y)));
			vs.push(Vertex::new(v3, normal, uv_max));
			
			is.extend(&[i+0, i+2, i+1]);
			is.extend(&[i+2, i+3, i+1]);
		}
		
		vs.reserve(24);
		is.reserve(36);
		
		let ux = 1.0 / 3.0;
		let uy = 1.0 / 2.0;
		let he = half_extents;
		gen_quad(vs, is, // F
			Vec3::new(-he.x,  he.y,  he.z),
			Vec3::new( he.x,  he.y,  he.z),
			Vec3::new(-he.x, -he.y,  he.z),
			Vec2::new(0.0, 0.0),
			Vec2::new(ux, uy));
		gen_quad(vs, is, // B
			Vec3::new( he.x,  he.y, -he.z),
			Vec3::new(-he.x,  he.y, -he.z),
			Vec3::new( he.x, -he.y, -he.z),
			Vec2::new(ux*2.0, 0.0),
			Vec2::new(1.0, uy));
		gen_quad(vs, is, // L
			Vec3::new(-he.x,  he.y, -he.z),
			Vec3::new(-he.x,  he.y,  he.z),
			Vec3::new(-he.x, -he.y, -he.z),
			Vec2::new(0.0, uy),
			Vec2::new(ux, 1.0));
		gen_quad(vs, is, // R
			Vec3::new( he.x,  he.y,  he.z),
			Vec3::new( he.x,  he.y, -he.z),
			Vec3::new( he.x, -he.y,  he.z),
			Vec2::new(ux*2.0, uy),
			Vec2::new(1.0, 1.0));
		gen_quad(vs, is, // U
			Vec3::new(-he.x,  he.y, -he.z),
			Vec3::new( he.x,  he.y, -he.z),
			Vec3::new(-he.x,  he.y,  he.z),
			Vec2::new(ux, 0.0),
			Vec2::new(ux*2.0, uy));
		gen_quad(vs, is, // D
			Vec3::new(-he.x, -he.y,  he.z),
			Vec3::new( he.x, -he.y,  he.z),
			Vec3::new(-he.x, -he.y, -he.z),
			Vec2::new(ux, uy),
			Vec2::new(ux*2.0, 1.0));
	}
}
