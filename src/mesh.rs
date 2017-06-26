use gl;
use gl::types::*;
use std::mem;
use std::ptr;
use std::str;
use cgmath;
use cgmath::prelude::*;
use cgmath::{Matrix4, Vector3, Deg, Basis3, Vector2};
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
    pub ibo: IBO,
	pub tex: Texture,

	pub geom_type: GeometryType,
	pub num_verts: u32,
	
	pub transform: cgmath::Decomposed<Vector3<GLfloat>, Basis3<GLfloat>>,
}

impl Mesh {
	pub fn new(vertex_shader: &str, fragment_shader: &str, 
			   vertex_data: &[Vector3<GLfloat>],
			   normal_data: &[Vector3<GLfloat>],
               index_data: &[i32],
			   texcoord_data: &[Vector2<GLfloat>],
			   image: &Image,
			   geom_type: GeometryType) -> Mesh {

		let program = Program::new(vertex_shader, fragment_shader);
		
		let tex = Texture::new(image);
	
		let verts = VBO::new(vertex_data).unwrap();
		let normals = VBO::new(normal_data).unwrap();
		let texcoords = VBO::new(texcoord_data).unwrap();
		let vao = VAO::new(verts, normals, texcoords, &program);

        let ibo = IBO::new(index_data).unwrap();
		
		let transform = cgmath::Decomposed::<Vector3<GLfloat>, Basis3<GLfloat>> {
			scale: 1.0,
			rot: Basis3::from_angle_x(Deg(-90.0)),
			disp: Vector3::new(0.0, 0.0, -2.75),
		};

		Mesh { program, vao, ibo, tex, num_verts: index_data.len() as u32, geom_type, transform }
	}
	
	pub fn draw(&mut self, proj: &Matrix4<GLfloat>) {
		let trans = Matrix4::from(self.transform);
		let view = <cgmath::Matrix4<f32> as One>::one();
		
        // TODO: use glVertexAttribFormat, glVertexAttribBinding, and glBindVertexBuffers
		self.vao.bind();
		
		self.tex.bind();
		
		self.ibo.bind();

		self.program.bind();
		self.program.bind_uniform_matrix4("trans", &trans);
		self.program.bind_uniform_matrix4("proj", &proj);
		self.program.bind_uniform_matrix4("view", &view);
		self.program.bind_uniform_int32("tex", self.tex.id as i32);
			
		let geom_type = match self.geom_type {
			GeometryType::Tris => gl::TRIANGLES,
			GeometryType::Quads => gl::QUADS,
		};
			
		unsafe {
            gl::DrawElements(geom_type, self.num_verts as i32, gl::UNSIGNED_INT, ptr::null());
		};
		
		self.transform.rot = self.transform.rot * Basis3::from_angle_z(Deg(-0.375));
	}
}
