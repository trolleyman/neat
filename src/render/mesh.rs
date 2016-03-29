use std::mem;

use glium::backend::Facade;
use glium::{IndexBuffer, VertexBuffer};
use glium::index;

use cgmath::{vec3, EuclideanVector, Vector3};

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

const LERP: f32 = 0.5;

pub struct Mesh {
	vertex_buffer: VertexBuffer<SimpleVertex>,
	index_buffer: IndexBuffer<u32>,
}
impl Mesh {
	pub fn sphere<F: Facade>(facade: &F) -> Mesh {
		const DETAIL: u32 = 0;
		
		let mut vs: Vec<SimpleVertex> = Vec::new();
		let mut is: Vec<u32> = Vec::new();
		
		Mesh::gen_dodec(&mut vs, &mut is, DETAIL);
		
		let vs = VertexBuffer::immutable(facade, &vs).unwrap();
		let is = IndexBuffer::immutable(facade, index::PrimitiveType::TrianglesList, &is).unwrap();
		
		Mesh{
			vertex_buffer: vs,
			index_buffer : is,
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
		
		Mesh::gen_dodec_face(vs, is, detail, v0, v1, v2);
		Mesh::gen_dodec_face(vs, is, detail, v0, v1, v2);
		Mesh::gen_dodec_face(vs, is, detail, v0, v1, v2);
		Mesh::gen_dodec_face(vs, is, detail, v0, v1, v2);
		Mesh::gen_dodec_face(vs, is, detail, v0, v1, v2);
	}
	
	fn gen_dodec_face(vs: &mut Vec<SimpleVertex>, is: &mut Vec<u32>, detail: u32, v0: Vector3<f32>, v1: Vector3<f32>, v2: Vector3<f32>) {
		// Generate base tri
		let (v0, v1, v2) = Mesh::gen_tri(vs, is, v0, v1, v2);
		if detail < 1 {
			return;
		}
		
		// Gen other tris
		Mesh::gen_dodec_recursive(vs, is, v0, v1, v2, detail, 1);
	}
	
	fn gen_dodec_recursive(vs: &mut Vec<SimpleVertex>, is: &mut Vec<u32>, v0: u32, v1: u32, v2: u32, detail: u32, level: u32) {
		let v01 = Vector3::from(vs[v0 as usize].pos).lerp(Vector3::from(vs[v1 as usize].pos), LERP);
		let v12 = Vector3::from(vs[v1 as usize].pos).lerp(Vector3::from(vs[v2 as usize].pos), LERP);
		let v20 = Vector3::from(vs[v2 as usize].pos).lerp(Vector3::from(vs[v0 as usize].pos), LERP);
		
		// Gen centre tri
		let (v01, v12, v20) = Mesh::gen_tri(vs, is, v01, v12, v20);
		
		if level >= detail {
			is.extend(&[v0 , v01, v20]); // Top sub-tri
			is.extend(&[v01, v1 , v12]); // Bottom left sub-tri
			is.extend(&[v20, v12, v2 ]); // Bottom right sub-tri
			return;
		}
		
		// Gen other sub-tris
		Mesh::gen_dodec_recursive(vs, is, v0 , v01, v20, detail, level + 1); // Top sub-tri
		Mesh::gen_dodec_recursive(vs, is, v01, v1 , v12, detail, level + 1); // Bottom left sub-tri
		Mesh::gen_dodec_recursive(vs, is, v20, v12, v2 , detail, level + 1); // Bottom right sub-tri
	}
	
	fn gen_tri(vs: &mut Vec<SimpleVertex>, is: &mut Vec<u32>, v0: Vector3<f32>, v1: Vector3<f32>, v2: Vector3<f32>) -> (u32, u32, u32) {
		// Push triangle
		let i = vs.len() as u32;
		vs.push(v0.into());
		vs.push(v1.into());
		vs.push(v2.into());
		let v0 = i;
		let v1 = i + 1;
		let v2 = i + 2;
		is.push(v0);
		is.push(v1);
		is.push(v2);
		(v0, v1, v2)
	}
	
	pub fn vertices(&self) -> &VertexBuffer<SimpleVertex> {
		&self.vertex_buffer
	}
	
	pub fn indices(&self) -> &IndexBuffer<u32> {
		&self.index_buffer
	}
}
