use cgmath::{Vector2, Vector3, Matrix3};

pub struct Texture {
	pub size: Vector2<u16>,
	pub pixels: Box<[Vector3<u8>]>,
}

pub struct Material {
	pub name: String,
	pub shader_vertex: String,
	pub shader_fragment: String,
	pub normals: Option<Texture>,
	pub diffuse: Option<Texture>,
	pub specular: Option<Texture>,
}

pub struct Triangle {
	pub indices: [u32; 3],
	pub material: u8,
}

pub struct Model {
	pub name: String,
	pub transform: Matrix3<f32>,
	pub materials: Box<[Material]>,
	pub vertices: Box<[Vector3<f32>]>,
	pub texcoords: Box<[Vector2<f32>]>,
	pub triangles: Box<[Triangle]>,
}
