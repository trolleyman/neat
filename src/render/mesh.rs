use std::mem;
use std::rc::Rc;
use std::process::exit;

use glium::backend::Context;
use glium::{IndexBuffer, VertexBuffer};
use glium::index;

use cgmath::{vec3, EuclideanVector, Vector3, Matrix4};

use render::{Render, Color};

#[derive(Copy, Clone, Debug)]
pub struct SimpleVertex {
	pub pos: [f32; 3],
}
impl From<Vector3<f32>> for SimpleVertex {
	fn from(v: Vector3<f32>) -> SimpleVertex {
		SimpleVertex{
			pos: unsafe { mem::transmute(v) },
		}
	}
}

implement_vertex!(SimpleVertex, pos);

#[derive(Debug)]
pub struct Mesh {
	vertex_buffer: VertexBuffer<SimpleVertex>,
	index_buffer: IndexBuffer<u32>,
}
impl Mesh {
	pub fn render(&self, r: &mut Render, model: Matrix4<f32>, color: Color) {
		r.render_simple(&self.vertex_buffer, &self.index_buffer, model, color);
	}
}
impl Mesh {
	pub fn sphere(ctx: &Rc<Context>, detail: u32) -> Mesh {
		let mut vs: Vec<SimpleVertex> = Vec::new();
		let mut is: Vec<u32> = Vec::new();
		
		Mesh::gen_sphere(&mut vs, &mut is, detail);
		Mesh::from_vecs(ctx, vs, is)
	}
	
	pub fn dodecahedron(ctx: &Rc<Context>) -> Mesh {
		let mut vs: Vec<SimpleVertex> = Vec::new();
		let mut is: Vec<u32> = Vec::new();
		
		Mesh::gen_dodec(&mut vs, &mut is, 0);
		Mesh::from_vecs(ctx, vs, is)
	}
	
	pub fn cube(ctx: &Rc<Context>) -> Mesh {
		let mut vs: Vec<SimpleVertex> = Vec::new();
		let mut is: Vec<u32> = Vec::new();
		
		Mesh::gen_cube(&mut vs, &mut is);
		Mesh::from_vecs(ctx, vs, is)
	}
	
	fn from_vecs(ctx: &Rc<Context>, vs: Vec<SimpleVertex>, is: Vec<u32>) -> Mesh {
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
	
	fn gen_cube(vs: &mut Vec<SimpleVertex>, is: &mut Vec<u32>) {
		fn push_quad(is: &mut Vec<u32>, i: u32, v0: u32, v1: u32, v2: u32, v3: u32) {
			is.extend(&[i+v0, i+v2, i+v1]);
			is.extend(&[i+v0, i+v3, i+v2]);
		}
		let i = vs.len() as u32;
		
		let cube_vs: &[SimpleVertex] = &[
			vec3(-0.5,  0.5, -0.5).into(),
			vec3( 0.5,  0.5, -0.5).into(),
			vec3( 0.5, -0.5, -0.5).into(),
			vec3(-0.5, -0.5, -0.5).into(),
			vec3(-0.5,  0.5,  0.5).into(),
			vec3( 0.5,  0.5,  0.5).into(),
			vec3( 0.5, -0.5,  0.5).into(),
			vec3(-0.5, -0.5,  0.5).into(),
		];
		
		vs.extend(cube_vs);
		push_quad(is, i, 0, 1, 2, 3); // F
		push_quad(is, i, 5, 4, 7, 6); // B
		push_quad(is, i, 0, 3, 7, 4); // L
		push_quad(is, i, 1, 2, 6, 5); // R
		push_quad(is, i, 0, 1, 5, 4); // U
		push_quad(is, i, 2, 3, 7, 6); // D
	}
	
	fn gen_sphere(vs: &mut Vec<SimpleVertex>, is: &mut Vec<u32>, detail: u32) {
		// Generate dodecohedron
		Mesh::gen_dodec(vs, is, detail);
		
		// Now scale vertices to proper locations.
		// (by normalising them)
		for v in vs.iter_mut() {
			*v = SimpleVertex::from(Vector3::from(v.pos).normalize());
		}
	}
	
	fn gen_dodec(vs: &mut Vec<SimpleVertex>, is: &mut Vec<u32>, detail: u32) {
		// v0 is top
		// v1 through v4 are vertices going anti-clockwise (looking down) around the dodecahedron
		// v5 is bottom
		let v0 = vec3( 0.0,  0.5,  0.0);
		let v1 = vec3( 0.0,  0.0,  0.5);
		let v2 = vec3( 0.5,  0.0,  0.0);
		let v3 = vec3( 0.0,  0.0, -0.5);
		let v4 = vec3(-0.5,  0.0,  0.0);
		let v5 = vec3( 0.0, -0.5,  0.0);
		
		let start_len = vs.len() as u32;
		
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
		
		let tris_per_face = (vs.len() as u32 - start_len) / 8;
		
		for face in 0..8 { // Generate index buffer
			let i = tris_per_face * face + start_len;
			Mesh::gen_dodec_face_inds(is, detail, i);
		}
	}
	
	fn gen_dodec_face_tris(vs: &mut Vec<SimpleVertex>, detail: u32, v0: Vector3<f32>, v1: Vector3<f32>, v2: Vector3<f32>) {
		let rows = 2u32.pow(detail) + 1;
		for row in 0..rows {
			// Create row + 1 vertices.
			let k_row = row as f32 / (rows - 1) as f32;
			let start = v0.lerp(v1, k_row); // Pos of start of row
			let end   = v0.lerp(v2, k_row); // Pos of end of row
			
			let cols = row + 1;
			for col in 0..cols {
				let k_col = if cols != 1 { col as f32 / (cols - 1) as f32 } else { 0.5 };
				let v = start.lerp(end, k_col);
				vs.push(v.into());
			}
		}
	}
	
	fn gen_dodec_face_inds(is: &mut Vec<u32>, detail: u32, offset: u32) {
		let mut prev_start = 0;
		let rows = 2u32.pow(detail) + 1;
		for row in 1..rows {
			let start = prev_start + row;
			
			is.push(offset+prev_start);
			is.push(offset+start);
			is.push(offset+start+1);
			
			for i in 0..row - 1 {
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
	
	pub fn vertices(&self) -> &VertexBuffer<SimpleVertex> {
		&self.vertex_buffer
	}
	
	pub fn indices(&self) -> &IndexBuffer<u32> {
		&self.index_buffer
	}
}
