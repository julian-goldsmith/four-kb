use std::ptr;
use gl;
use gl::types::*;
use cgmath;
use cgmath::prelude::*;
use cgmath::{Matrix4, Vector3, Basis3, Vector2, Decomposed, PerspectiveFov};
use gfx::image::Image;
use gfx::lowlevel::*;
use gfx::lowlevel::program::Program;

pub struct Mesh {
	pub program: Program,
    pub ibo: IBO,
	pub vao: VAO,
	pub tex: Texture,
	pub normal_tex: Texture,
    pub transform: Matrix4<f32>,

	pub num_verts: u32,
}

impl Mesh {
	pub fn new(program: Program, 
               index_data: &[u32],
			   vertex_data: &[Vector3<GLfloat>],
			   normals: &Image,
			   texcoord_data: &[Vector2<GLfloat>],
			   image: &Image,
               transform: Matrix4<f32>) -> Mesh {

		let tex = Texture::new(image);
		let normal_tex = Texture::new(normals);
		
        let ibo = IBO::new(index_data).unwrap();
		let verts = VBO::new(vertex_data).unwrap();
		let texcoords = VBO::new(texcoord_data).unwrap();
		let vao = VAO::new(verts, texcoords, &program);

		Mesh { program, ibo, vao, tex, normal_tex, transform, num_verts: index_data.len() as u32, }
	}
	
	pub fn draw(&self, view: &Decomposed<Vector3<GLfloat>, Basis3<GLfloat>>, proj: &Matrix4<f32>) {
		let view = view.clone().into();

        self.ibo.bind();
		
        // TODO: use glVertexAttribFormat, glVertexAttribBinding, and glBindVertexBuffers
		self.vao.bind();
		
		self.tex.bind();
		self.normal_tex.bind();

		self.program.bind();
		self.program.bind_uniform_matrix4("trans", &self.transform);
		self.program.bind_uniform_matrix4("proj", &proj);
		self.program.bind_uniform_matrix4("view", &view);
		self.program.bind_uniform_int32("tex", self.tex.tex_unit as i32);
		self.program.bind_uniform_int32("normal_tex", self.normal_tex.tex_unit as i32);
			
		unsafe {
			gl::DrawElements(gl::TRIANGLES, self.num_verts as i32, gl::UNSIGNED_INT, ptr::null_mut());
		};
	}
}
