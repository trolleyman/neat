use prelude::*;
use std::rc::Rc;
use std::process::exit;
use std::mem;

use glium::index;
use glium::{Texture2d, IndexBuffer, VertexBuffer};

use render::{RenderableMesh, Material, Render};
use util;

#[derive(Copy, Clone, Debug)]
pub struct LitVertex {
	pos   : [f32; 3],
	normal: [f32; 3],
	uv    : [f32; 2],
}
implement_vertex!(LitVertex, pos, normal, uv);

impl LitVertex {
	pub fn new(pos: Vector3<f32>, normal: Vector3<f32>, uv: Vector2<f32>) -> LitVertex {
		LitVertex {
			pos   : unsafe { mem::transmute(pos) },
			normal: unsafe { mem::transmute(normal) },
			uv    : unsafe { mem::transmute(uv) },
		}
	}
}

/// A LitMesh is a textured mesh that is affected by lighting.
pub struct LitMesh {
	/// The list of vertices.
	vertex_buffer: VertexBuffer<LitVertex>,
	/// The list of triangles that make the mesh up. Stored in counter-clockwise order.
	index_buffer : IndexBuffer<u16>,
	/// The texture that will be used to texture the object.
	texture      : Rc<Texture2d>,
	/// The material that the object has.
	material     : Material,
}
impl RenderableMesh for LitMesh {
	fn render(&self, r: &mut Render, model: Matrix4<f32>) {
		r.render_lit(&self.vertex_buffer, &self.index_buffer, model, &*self.texture, &self.material);
	}
}
impl LitMesh {
	/// Generates a new sphere with a specified detail, texture and material.
	/// 
	/// At the moment the uvs of the mesh outputted are all set to 0.0,0.0.
	pub fn sphere(ctx: &Rc<Context>, detail: u32, texture: Rc<Texture2d>, material: Material) -> LitMesh {
		let mut vs: Vec<LitVertex> = Vec::new();
		let mut is: Vec<u16> = Vec::new();
		
		LitMesh::gen_sphere(&mut vs, &mut is, detail);
		LitMesh::from_vecs(ctx, vs, is, texture, material)
	}
	
	/// Generates a cuboid with the specified half extents, texture and material.
	/// 
	/// The uvs are layed out like this:
	/// ```
	///   0           1
	/// 0 +---+---+---+
	///   | F | U | B | // Front, Up, Back
	///   +---+---+---+
	///   | L | D | R | // Left, Down, Right
	/// 1 +---+---+---+
	/// ```
	pub fn cuboid(ctx: &Rc<Context>, half_extents: Vector3<f32>, texture: Rc<Texture2d>, material: Material) -> LitMesh {
		let mut vs: Vec<LitVertex> = Vec::new();
		let mut is: Vec<u16> = Vec::new();
		
		LitMesh::gen_cuboid(&mut vs, &mut is, half_extents);
		LitMesh::from_vecs(ctx, vs, is, texture, material)
	}
	
	fn from_vecs(ctx: &Rc<Context>, vs: Vec<LitVertex>, is: Vec<u16>, texture: Rc<Texture2d>, material: Material) -> LitMesh {
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
		
		LitMesh {
			vertex_buffer: vs,
			index_buffer : is,
			texture      : texture,
			material     : material,
		}
	}
	
	fn gen_sphere(vs: &mut Vec<LitVertex>, is: &mut Vec<u16>, detail: u32) {
		let start = vs.len();
		LitMesh::gen_dodec(vs, is, detail);
		for i in start..vs.len() {
			Vector3::<f32>::from(vs[i].pos).normalize_mut();
			vs[i].normal = vs[i].pos;
		}
	}
	
	fn gen_dodec(vs: &mut Vec<LitVertex>, is: &mut Vec<u16>, detail: u32) {
		// v0 is top
		// v1 through v4 are vertices going anti-clockwise (looking down) around the dodecahedron
		// v5 is bottom
		let v0 = Vector3::new( 0.0,  0.5,  0.0);
		let v1 = Vector3::new( 0.0,  0.0,  0.5);
		let v2 = Vector3::new( 0.5,  0.0,  0.0);
		let v3 = Vector3::new( 0.0,  0.0, -0.5);
		let v4 = Vector3::new(-0.5,  0.0,  0.0);
		let v5 = Vector3::new( 0.0, -0.5,  0.0);
		
		let start_len = vs.len() as u16;
		
		// Top half
		LitMesh::gen_dodec_face_tris(vs, detail, v0, v1, v2);
		LitMesh::gen_dodec_face_tris(vs, detail, v0, v2, v3);
		LitMesh::gen_dodec_face_tris(vs, detail, v0, v3, v4);
		LitMesh::gen_dodec_face_tris(vs, detail, v0, v4, v1);
		// Bottom half
		LitMesh::gen_dodec_face_tris(vs, detail, v5, v4, v3);
		LitMesh::gen_dodec_face_tris(vs, detail, v5, v3, v2);
		LitMesh::gen_dodec_face_tris(vs, detail, v5, v2, v1);
		LitMesh::gen_dodec_face_tris(vs, detail, v5, v1, v4);
		
		let tris_per_face = (vs.len() as u16 - start_len) / 8;
		
		for face in 0..8 { // Generate index buffer
			let i = tris_per_face * face + start_len;
			LitMesh::gen_dodec_face_inds(is, detail, i);
		}
	}
	
	fn gen_dodec_face_tris(vs: &mut Vec<LitVertex>, detail: u32, v0: Vector3<f32>, v1: Vector3<f32>, v2: Vector3<f32>) {
		let normal = (v1 - v0).cross(&(v2 - v0));
		let rows = 2u32.pow(detail) + 1;
		for row in 0..rows {
			// Create row + 1 vertices.
			let k_row = row as f32 / (rows - 1) as f32;
			let start = util::lerp(v0, v1, k_row); // Pos of start of row
			let end   = util::lerp(v0, v2, k_row); // Pos of end of row
			
			let cols = row + 1;
			for col in 0..cols {
				let k_col = if cols != 1 { col as f32 / (cols - 1) as f32 } else { 0.5 };
				let v = util::lerp(start, end, k_col);
				vs.push(LitVertex::new(v, normal, Vector2::new(0.0, 0.0)));
			}
		}
	}
	
	fn gen_dodec_face_inds(is: &mut Vec<u16>, detail: u32, offset: u16) {
		let mut prev_start = 0;
		let rows = 2u32.pow(detail) + 1;
		for row in 1..rows {
			let start = prev_start + row as u16;
			
			is.push(offset+prev_start);
			is.push(offset+start);
			is.push(offset+start+1);
			
			for i in 0..row - 1 {
				let i = i as u16;
				// Triangle pointing down
				is.push(offset+i+prev_start+1);
				is.push(offset+i+prev_start);
				is.push(offset+i+start+1);
				// Triangle pointing up
				is.push(offset+i+prev_start+1);
				is.push(offset+i+start+1);
				is.push(offset+i+start+2);
			}
			prev_start = start;
		}
	}
	
	fn gen_cuboid(vs: &mut Vec<LitVertex>, is: &mut Vec<u16>, half_extents: Vector3<f32>) {
		// v0 --- v1 
		// |          <- Looking forward, normal out of the screen.
		// v2     v3  
		fn gen_quad(vs: &mut Vec<LitVertex>, is: &mut Vec<u16>, v0: Vector3<f32>, v1: Vector3<f32>, v2: Vector3<f32>, uv_min: Vector2<f32>, uv_max: Vector2<f32>) {
			let i = vs.len() as u16;
			let v02 = v2-v0;
			let v01 = v1-v0;
			let normal = v02.cross(&v01).normalize();
			let v3 = v0 + v01 + v02;
			
			vs.push(LitVertex::new(v0, normal, uv_min));
			vs.push(LitVertex::new(v1, normal, Vector2::new(uv_max.x, uv_min.y)));
			vs.push(LitVertex::new(v2, normal, Vector2::new(uv_min.x, uv_max.y)));
			vs.push(LitVertex::new(v3, normal, uv_max));
			
			is.extend(&[i+0, i+2, i+1]);
			is.extend(&[i+2, i+3, i+1]);
		}
		
		vs.reserve(24);
		is.reserve(36);
		
		let ux = 1.0 / 3.0;
		let uy = 1.0 / 2.0;
		let he = half_extents;
		gen_quad(vs, is, // F
			Vector3::new(-he.x,  he.y,  he.z),
			Vector3::new( he.x,  he.y,  he.z),
			Vector3::new(-he.x, -he.y,  he.z),
			Vector2::new(0.0, 0.0),
			Vector2::new(ux, uy));
		gen_quad(vs, is, // B
			Vector3::new( he.x,  he.y, -he.z),
			Vector3::new(-he.x,  he.y, -he.z),
			Vector3::new( he.x, -he.y, -he.z),
			Vector2::new(ux*2.0, 0.0),
			Vector2::new(1.0, uy));
		gen_quad(vs, is, // L
			Vector3::new(-he.x,  he.y, -he.z),
			Vector3::new(-he.x,  he.y,  he.z),
			Vector3::new(-he.x, -he.y, -he.z),
			Vector2::new(0.0, uy),
			Vector2::new(ux, 1.0));
		gen_quad(vs, is, // R
			Vector3::new( he.x,  he.y,  he.z),
			Vector3::new( he.x,  he.y, -he.z),
			Vector3::new( he.x, -he.y,  he.z),
			Vector2::new(ux*2.0, uy),
			Vector2::new(1.0, 1.0));
		gen_quad(vs, is, // U
			Vector3::new(-he.x,  he.y, -he.z),
			Vector3::new( he.x,  he.y, -he.z),
			Vector3::new(-he.x,  he.y,  he.z),
			Vector2::new(ux, 0.0),
			Vector2::new(ux*2.0, uy));
		gen_quad(vs, is, // D
			Vector3::new(-he.x, -he.y,  he.z),
			Vector3::new( he.x, -he.y,  he.z),
			Vector3::new(-he.x, -he.y, -he.z),
			Vector2::new(ux, uy),
			Vector2::new(ux*2.0, 1.0));
	}
}
