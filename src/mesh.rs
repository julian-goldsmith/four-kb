use gl;
use gl::types::*;
use cgmath;
use cgmath::prelude::*;
use cgmath::{Matrix4, Vector3, Basis3, Vector2};
use image::Image;
use gfx::*;
use gfx::program::Program;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GeometryType {
	Tris,
	Quads,
}

pub struct Mesh {
	pub program: Program,
	pub vao: VAO,
	pub tex: Texture,
	pub normal_tex: Texture,

	pub geom_type: GeometryType,
	pub num_verts: u32,
}

impl Mesh {
	pub fn new(program: Program, 
			   vertex_data: &[Vector3<GLfloat>],
			   normals: &Image,
               index_data: &[i32],
			   texcoord_data: &[Vector2<GLfloat>],
			   image: &Image,
			   geom_type: GeometryType) -> Mesh {

		let tex = Texture::new(image);
		let normal_tex = Texture::new(normals);
		
		let verts = VBO::new(vertex_data).unwrap();
		let texcoords = VBO::new(texcoord_data).unwrap();
		let vao = VAO::new(verts, texcoords, &program);

		Mesh { program, vao, tex, normal_tex, num_verts: index_data.len() as u32, geom_type, }
	}
	
	pub fn draw(&self, proj: &Matrix4<GLfloat>, transform: &cgmath::Decomposed<Vector3<GLfloat>, Basis3<GLfloat>>) {
		let trans = transform.clone().into();
		let view = <cgmath::Matrix4<f32> as One>::one();
		
        // TODO: use glVertexAttribFormat, glVertexAttribBinding, and glBindVertexBuffers
		self.vao.bind();
		
		self.tex.bind();
		self.normal_tex.bind();

		self.program.bind();
		self.program.bind_uniform_matrix4("trans", &trans);
		self.program.bind_uniform_matrix4("proj", proj);
		self.program.bind_uniform_matrix4("view", &view);
		self.program.bind_uniform_int32("tex", self.tex.tex_unit as i32);
		self.program.bind_uniform_int32("normal_tex", self.normal_tex.tex_unit as i32);
			
		let geom_type = match self.geom_type {
			GeometryType::Tris => gl::TRIANGLES,
			GeometryType::Quads => gl::QUADS,
		};
			
		unsafe {
			gl::DrawArrays(geom_type, 0, self.num_verts as i32);
		};

        println!("Drew {} verts", self.num_verts);
	}
}
