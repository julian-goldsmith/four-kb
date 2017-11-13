use std::ptr;
use gl;
use gl::types::*;
use cgmath;
use cgmath::prelude::*;
use cgmath::{Matrix4, Vector3, Basis3, Vector2, Decomposed, PerspectiveFov};
use gfx::image::Image;
use gfx::lowlevel::*;
use gfx::material::Material;

pub struct Mesh {
    pub materials: Vec<Material>,

    pub ibo: IBO,
	pub vao: VAO,
    pub transform: Matrix4<f32>,

	pub num_verts: u32,
}

impl Mesh {
	pub fn new(program: Program, 
               index_data: &[u32],
			   vertex_data: &[Vector3<GLfloat>],
			   normals: &[Vector3<GLfloat>],
			   texcoord_data: &[Vector2<GLfloat>],
			   image: &Image,
               normal_map: &Image,
               transform: Matrix4<f32>) -> Mesh {
		
        let ibo = IBO::new(index_data).unwrap();
		let verts = VBO::new(vertex_data).unwrap();
        let norms = VBO::new(normals).unwrap();
		let texcoords = VBO::new(texcoord_data).unwrap();
		let vao = VAO::new(verts, norms, texcoords, &program);

        let materials = vec![Material::new(program, image, normal_map)];

		Mesh { ibo, vao, materials, transform, num_verts: index_data.len() as u32, }
	}
	
	pub fn draw(&self, view: &Decomposed<Vector3<GLfloat>, Basis3<GLfloat>>, proj: &Matrix4<f32>) {
		let view: Matrix4<f32> = view.clone().into();

        self.ibo.bind();
		
        // TODO: use glVertexAttribFormat, glVertexAttribBinding, and glBindVertexBuffers
		self.vao.bind();

        let material = &self.materials[0];
		
        let uniforms = [
            Uniform { name: "trans", value: &self.transform as &Uniformable },
            Uniform { name: "proj", value: proj as &Uniformable },
            Uniform { name: "view", value: &view as &Uniformable },
            
            // FIXME: these were i32s, make sure u32s are correct
            Uniform { name: "tex", value: &(material.diffuse_tex.tex_unit as i32) as &Uniformable },
            Uniform { name: "normal_tex", value: &(material.normal_tex.tex_unit as i32) as &Uniformable },
        ];

        material.bind(&uniforms);
		
		unsafe {
			gl::DrawElements(gl::TRIANGLES, self.num_verts as i32, gl::UNSIGNED_INT, ptr::null_mut());
		};
	}
}
