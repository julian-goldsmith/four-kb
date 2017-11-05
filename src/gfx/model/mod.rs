use std::iter::Iterator;
use std::path::Path;
use cgmath::{Vector2, Vector3, Matrix4};
use gfx::mesh::Mesh;
use gfx::image;
use gfx::lowlevel::program::Program;

pub mod model_loader;

#[derive(Debug)]
pub struct Texture {
	pub size: Vector2<u16>,
	pub pixels: Box<[Vector3<u8>]>,
}

#[derive(Debug)]
pub struct Material {
	pub name: String,
	pub shader_vertex: String,
	pub shader_fragment: String,
	pub normals: Option<Texture>,
	pub diffuse: Option<Texture>,
	pub specular: Option<Texture>,
}

#[derive(Debug)]
pub struct Model {
	pub name: String,
	pub transform: Matrix4<f32>,
	pub materials: Box<[Material]>,
    pub indices: Box<[u32]>,
	pub vertices: Box<[Vector3<f32>]>,
	pub texcoords: Box<[Vector2<f32>]>,
}

impl From<Model> for Mesh {
    fn from(model: Model) -> Mesh {
        let image = image::load_image(&Path::new("assets/levi_displacement.png")).unwrap();
        let normals = image::load_image(&Path::new("assets/levi_normals.png")).unwrap();
		let program = Program::from_path(&Path::new("assets/shader.vert"), &Path::new("assets/shader.frag"));

        Mesh::new(program, &model.indices[0..], &model.vertices[0..], &normals, &model.texcoords[0..], &image, model.transform)
    }
}
