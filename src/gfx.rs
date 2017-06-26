use gl;
use gl::types::*;
use std::mem;
use std::ptr;
use std::str;
use std::ffi::CString;
use std::rc::Rc;

pub struct VBO {
	pub id: GLuint,
}

impl VBO {
	pub fn new<T>(data: &[T]) -> Option<VBO> {
		let mut vbo = 0;

		if data.len() == 0 {
			return None;
		}
		
		unsafe {
			gl::GenBuffers(1, &mut vbo);
			gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
			gl::BufferData(gl::ARRAY_BUFFER,
						   (data.len() * mem::size_of::<T>()) as GLsizeiptr,
						   mem::transmute(&data[0]),
						   gl::STATIC_DRAW);
		};
		
		Some(VBO { id: vbo })
	}
}

impl Drop for VBO {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteBuffers(1, &self.id);
		}
	}
}

pub struct IBO {
	pub id: GLuint,
}

impl IBO {
	pub fn new(data: &[i32]) -> Option<IBO> {
		let mut ibo = 0;

		if data.len() == 0 {
			return None;
		}
		
		unsafe {
			gl::GenBuffers(1, &mut ibo);
			gl::BindBuffer(gl::ARRAY_BUFFER, ibo);
			gl::BufferData(gl::ARRAY_BUFFER,
						   (data.len() * mem::size_of::<i32>()) as GLsizeiptr,
						   mem::transmute(&data[0]),
						   gl::STATIC_DRAW);
		};
		
		Some(IBO { id: ibo })
	}
}

impl Drop for IBO {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteBuffers(1, &self.id);
		}
	}
}

pub struct VAO {
	pub id: GLuint,
	verts: VBO,
	normal: VBO,
	texcoords: VBO,
}

impl VAO {
	pub fn new(verts: VBO, normal: VBO, texcoords: VBO, program: GLuint) -> VAO {
		let mut vao = VAO { id: 0, verts, normal, texcoords };

		unsafe {
			gl::GenVertexArrays(1, &mut vao.id);
			gl::BindVertexArray(vao.id);
		};

		VAO::bind_attribute("position", &vao.verts, 3, program);
		VAO::bind_attribute("normal", &vao.normal, 3, program);
		VAO::bind_attribute("texcoord", &vao.texcoords, 2, program);

		VAO::set_frag_data_name("out_color", program);

		vao
	}
	
	pub fn bind(&self) {
		unsafe {
			gl::BindVertexArray(self.id);
		}
	}

	fn bind_attribute(name: &str, vbo: &VBO, num_components: u16, program: GLuint) {
		unsafe {
			let attr = gl::GetAttribLocation(program, CString::new(name).unwrap().as_ptr());
			gl::EnableVertexAttribArray(attr as GLuint);
			
			gl::BindBuffer(gl::ARRAY_BUFFER, vbo.id);
			gl::VertexAttribPointer(attr as GLuint,
									num_components as i32,
									gl::FLOAT,
									gl::FALSE as GLboolean,
									0,
									ptr::null());
		};
	}

	fn set_frag_data_name(name: &str, program: GLuint) {
		unsafe {
			gl::BindFragDataLocation(program, 0, CString::new(name).unwrap().as_ptr());
		};
	}
}

impl Drop for VAO {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteVertexArrays(1, &self.id);
		}
	}
}