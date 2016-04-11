use std::mem;
use std::rc::Rc;
use std::process::exit;
use std::convert::From;

use glium::backend::Context;
use glium::{IndexBuffer, VertexBuffer};
use glium::index;
use na::{Vec3, Mat4, Norm};

use render::{Render, Color};
use util;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
	pub pos: [f32; 3],
}
implement_vertex!(Vertex, pos);

impl From<Vec3<f32>> for Vertex {
	fn from(v: Vec3<f32>) -> Vertex {
		Vertex{
			pos: unsafe { mem::transmute(v) },
		}
	}
}

/// A simple mesh is basically just a list of triangles
#[derive(Debug)]
pub struct Mesh {
	vertex_buffer: VertexBuffer<Vertex>,
	index_buffer: IndexBuffer<u16>,
}
impl Mesh {
	pub fn render(&self, r: &mut Render, model: Mat4<f32>, color: Color) {
		r.render_simple(&self.vertex_buffer, &self.index_buffer, model, color);
	}
	
	pub fn sphere(ctx: &Rc<Context>, detail: u32) -> Mesh {
		let mut vs: Vec<Vertex> = Vec::new();
		let mut is: Vec<u16> = Vec::new();
		
		Mesh::gen_sphere(&mut vs, &mut is, detail);
		Mesh::from_vecs(ctx, vs, is)
	}
	
	pub fn dodecahedron(ctx: &Rc<Context>) -> Mesh {
		let mut vs: Vec<Vertex> = Vec::new();
		let mut is: Vec<u16> = Vec::new();
		
		Mesh::gen_dodec(&mut vs, &mut is, 0);
		Mesh::from_vecs(ctx, vs, is)
	}
	
	pub fn cuboid(ctx: &Rc<Context>, half_extents: Vec3<f32>) -> Mesh {
		let mut vs: Vec<Vertex> = Vec::new();
		let mut is: Vec<u16> = Vec::new();
		
		Mesh::gen_cuboid(&mut vs, &mut is, half_extents);
		Mesh::from_vecs(ctx, vs, is)
	}
	
	pub fn cube(ctx: &Rc<Context>) -> Mesh {
		let mut vs: Vec<Vertex> = Vec::new();
		let mut is: Vec<u16> = Vec::new();
		
		Mesh::gen_cube(&mut vs, &mut is);
		Mesh::from_vecs(ctx, vs, is)
	}
	
	fn from_vecs(ctx: &Rc<Context>, vs: Vec<Vertex>, is: Vec<u16>) -> Mesh {
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
		}
	}
	
	fn gen_cube(vs: &mut Vec<Vertex>, is: &mut Vec<u16>) {
		Mesh::gen_cuboid(vs, is, Vec3::new(0.5, 0.5, 0.5))
	}
	
	fn gen_cuboid(vs: &mut Vec<Vertex>, is: &mut Vec<u16>, half_extents: Vec3<f32>) {
		fn push_quad(is: &mut Vec<u16>, i: u16, v0: u16, v1: u16, v2: u16, v3: u16) {
			is.extend(&[i+v0, i+v2, i+v1]);
			is.extend(&[i+v0, i+v3, i+v2]);
		}
		
		let he = half_extents;
		let i = vs.len() as u16;
		let cuboid_vs: &[Vertex] = &[
			Vec3::new(-he.x,  he.y, -he.z).into(), // FUL
			Vec3::new( he.x,  he.y, -he.z).into(), // FUR
			Vec3::new( he.x, -he.y, -he.z).into(), // FDR
			Vec3::new(-he.x, -he.y, -he.z).into(), // FDL
			Vec3::new(-he.x,  he.y,  he.z).into(), // BUL
			Vec3::new( he.x,  he.y,  he.z).into(), // BUR
			Vec3::new( he.x, -he.y,  he.z).into(), // BDR
			Vec3::new(-he.x, -he.y,  he.z).into(), // BDL
		];
		
		vs.extend(cuboid_vs);
		push_quad(is, i, 0, 3, 2, 1); // F
		push_quad(is, i, 5, 6, 7, 4); // B
		push_quad(is, i, 0, 4, 7, 3); // L
		push_quad(is, i, 1, 2, 6, 5); // R
		push_quad(is, i, 1, 5, 4, 0); // U
		push_quad(is, i, 2, 3, 7, 6); // D
	}
	
	fn gen_sphere(vs: &mut Vec<Vertex>, is: &mut Vec<u16>, detail: u32) {
		// Generate dodecohedron
		Mesh::gen_dodec(vs, is, detail);
		
		// Now scale vertices to proper locations.
		// (by normalising them)
		for v in vs.iter_mut() {
			let pos: Vec3<f32> = {
				let add: &[f32;3] = &v.pos;
				let into: &Vec3<f32> = add.into();
				*into
			};
			*v = Vertex::from(pos.normalize());
		}
	}
	
	fn gen_dodec(vs: &mut Vec<Vertex>, is: &mut Vec<u16>, detail: u32) {
		// v0 is top
		// v1 through v4 are vertices going anti-clockwise (looking down) around the dodecahedron
		// v5 is bottom
		let v0 = Vec3::new( 0.0,  0.5,  0.0);
		let v1 = Vec3::new( 0.0,  0.0,  0.5);
		let v2 = Vec3::new( 0.5,  0.0,  0.0);
		let v3 = Vec3::new( 0.0,  0.0, -0.5);
		let v4 = Vec3::new(-0.5,  0.0,  0.0);
		let v5 = Vec3::new( 0.0, -0.5,  0.0);
		
		let start_len = vs.len() as u16;
		
		// Top half
		Mesh::gen_dodec_face_tris(vs, detail, v0, v1, v2);
		Mesh::gen_dodec_face_tris(vs, detail, v0, v2, v3);
		Mesh::gen_dodec_face_tris(vs, detail, v0, v3, v4);
		Mesh::gen_dodec_face_tris(vs, detail, v0, v4, v1);
		// Bottom half
		Mesh::gen_dodec_face_tris(vs, detail, v5, v4, v3);
		Mesh::gen_dodec_face_tris(vs, detail, v5, v3, v2);
		Mesh::gen_dodec_face_tris(vs, detail, v5, v2, v1);
		Mesh::gen_dodec_face_tris(vs, detail, v5, v1, v4);
		
		let tris_per_face = (vs.len() as u16 - start_len) / 8;
		
		for face in 0..8 { // Generate index buffer
			let i = tris_per_face * face + start_len;
			Mesh::gen_dodec_face_inds(is, detail, i);
		}
	}
	
	fn gen_dodec_face_tris(vs: &mut Vec<Vertex>, detail: u32, v0: Vec3<f32>, v1: Vec3<f32>, v2: Vec3<f32>) {
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
				vs.push(v.into());
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
	
	pub fn vertices(&self) -> &VertexBuffer<Vertex> {
		&self.vertex_buffer
	}
	
	pub fn indices(&self) -> &IndexBuffer<u16> {
		&self.index_buffer
	}
}
