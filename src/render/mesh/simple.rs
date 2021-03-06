use prelude::*;
use std::mem;
use std::rc::Rc;
use std::process::exit;

use glium::{IndexBuffer, VertexBuffer};
use glium::index;

use render::{Render, Color};
use util;

#[derive(Copy, Clone, Debug)]
pub struct SimpleVertex {
	pub pos: [f32; 3],
}
implement_vertex!(SimpleVertex, pos);

impl From<Vector3<f32>> for SimpleVertex {
	fn from(v: Vector3<f32>) -> SimpleVertex {
		SimpleVertex{
			pos: unsafe { mem::transmute(v) },
		}
	}
}

/// A simple mesh is a list of triangles.
/// 
/// It is not a RenderableMesh on its own. Use ColoredMesh to wrap it.
#[derive(Debug)]
pub struct SimpleMesh {
	/// The list of vertices
	vertex_buffer: VertexBuffer<SimpleVertex>,
	/// The list of triangles, in counter-clockwise order.
	index_buffer: IndexBuffer<u16>,
}
impl SimpleMesh {
	/// Render the mesh
	pub fn render(&self, r: &mut Render, model: Matrix4<f32>, color: Color) {
		r.render_simple(&self.vertex_buffer, &self.index_buffer, model, color);
	}
	
	/// Construct a new mesh that is an approximation of a sphere.
	/// 
	/// Takes a `detail` which specifies how much to subdivide the sphere.
	/// *Be warned:* The number of faces is proportional to 2^detail.
	///
	/// Detail 0 is the same as a dodecahedron.
	pub fn sphere(ctx: &Rc<Context>, detail: u32) -> SimpleMesh {
		let mut vs: Vec<SimpleVertex> = Vec::new();
		let mut is: Vec<u16> = Vec::new();
		
		SimpleMesh::gen_sphere(&mut vs, &mut is, detail);
		SimpleMesh::from_vecs(ctx, vs, is)
	}
	
	/// Construct a new mesh that is a dodecahedron.
	pub fn dodecahedron(ctx: &Rc<Context>) -> SimpleMesh {
		let mut vs: Vec<SimpleVertex> = Vec::new();
		let mut is: Vec<u16> = Vec::new();
		
		SimpleMesh::gen_dodec(&mut vs, &mut is, 0);
		SimpleMesh::from_vecs(ctx, vs, is)
	}
	
	/// Construct a cuboid from it's half extents.
	pub fn cuboid(ctx: &Rc<Context>, half_extents: Vector3<f32>) -> SimpleMesh {
		let mut vs: Vec<SimpleVertex> = Vec::new();
		let mut is: Vec<u16> = Vec::new();
		
		SimpleMesh::gen_cuboid(&mut vs, &mut is, half_extents);
		SimpleMesh::from_vecs(ctx, vs, is)
	}
	
	/// Construct a cube with size 1.0 on all sides.
	pub fn cube(ctx: &Rc<Context>) -> SimpleMesh {
		let mut vs: Vec<SimpleVertex> = Vec::new();
		let mut is: Vec<u16> = Vec::new();
		
		SimpleMesh::gen_cube(&mut vs, &mut is);
		SimpleMesh::from_vecs(ctx, vs, is)
	}
	
	fn from_vecs(ctx: &Rc<Context>, vs: Vec<SimpleVertex>, is: Vec<u16>) -> SimpleMesh {
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
		
		SimpleMesh {
			vertex_buffer: vs,
			index_buffer : is,
		}
	}
	
	fn gen_cube(vs: &mut Vec<SimpleVertex>, is: &mut Vec<u16>) {
		SimpleMesh::gen_cuboid(vs, is, Vector3::new(0.5, 0.5, 0.5))
	}
	
	fn gen_cuboid(vs: &mut Vec<SimpleVertex>, is: &mut Vec<u16>, half_extents: Vector3<f32>) {
		fn push_quad(is: &mut Vec<u16>, i: u16, v0: u16, v1: u16, v2: u16, v3: u16) {
			is.extend(&[i+v0, i+v2, i+v1]);
			is.extend(&[i+v0, i+v3, i+v2]);
		}
		
		let he = half_extents;
		let i = vs.len() as u16;
		let cuboid_vs: &[SimpleVertex] = &[
			Vector3::new(-he.x,  he.y, -he.z).into(), // FUL
			Vector3::new( he.x,  he.y, -he.z).into(), // FUR
			Vector3::new( he.x, -he.y, -he.z).into(), // FDR
			Vector3::new(-he.x, -he.y, -he.z).into(), // FDL
			Vector3::new(-he.x,  he.y,  he.z).into(), // BUL
			Vector3::new( he.x,  he.y,  he.z).into(), // BUR
			Vector3::new( he.x, -he.y,  he.z).into(), // BDR
			Vector3::new(-he.x, -he.y,  he.z).into(), // BDL
		];
		
		vs.extend(cuboid_vs);
		push_quad(is, i, 0, 3, 2, 1); // F
		push_quad(is, i, 5, 6, 7, 4); // B
		push_quad(is, i, 0, 4, 7, 3); // L
		push_quad(is, i, 1, 2, 6, 5); // R
		push_quad(is, i, 1, 5, 4, 0); // U
		push_quad(is, i, 2, 3, 7, 6); // D
	}
	
	fn gen_sphere(vs: &mut Vec<SimpleVertex>, is: &mut Vec<u16>, detail: u32) {
		// Generate dodecohedron
		SimpleMesh::gen_dodec(vs, is, detail);
		
		// Now scale vertices to proper locations.
		// (by normalising them)
		for v in vs.iter_mut() {
			let pos: Vector3<f32> = Vector3::new(v.pos[0], v.pos[1], v.pos[2]);
			*v = SimpleVertex::from(pos.normalize());
		}
	}
	
	fn gen_dodec(vs: &mut Vec<SimpleVertex>, is: &mut Vec<u16>, detail: u32) {
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
		SimpleMesh::gen_dodec_face_tris(vs, detail, v0, v1, v2);
		SimpleMesh::gen_dodec_face_tris(vs, detail, v0, v2, v3);
		SimpleMesh::gen_dodec_face_tris(vs, detail, v0, v3, v4);
		SimpleMesh::gen_dodec_face_tris(vs, detail, v0, v4, v1);
		// Bottom half
		SimpleMesh::gen_dodec_face_tris(vs, detail, v5, v4, v3);
		SimpleMesh::gen_dodec_face_tris(vs, detail, v5, v3, v2);
		SimpleMesh::gen_dodec_face_tris(vs, detail, v5, v2, v1);
		SimpleMesh::gen_dodec_face_tris(vs, detail, v5, v1, v4);
		
		let tris_per_face = (vs.len() as u16 - start_len) / 8;
		
		for face in 0..8 { // Generate index buffer
			let i = tris_per_face * face + start_len;
			SimpleMesh::gen_dodec_face_inds(is, detail, i);
		}
	}
	
	fn gen_dodec_face_tris(vs: &mut Vec<SimpleVertex>, detail: u32, v0: Vector3<f32>, v1: Vector3<f32>, v2: Vector3<f32>) {
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
	
	pub fn vertices(&self) -> &VertexBuffer<SimpleVertex> {
		&self.vertex_buffer
	}
	
	pub fn indices(&self) -> &IndexBuffer<u16> {
		&self.index_buffer
	}
}
