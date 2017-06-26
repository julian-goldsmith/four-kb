use gl;
use gl::types::*;
use std::ptr;
use std::str;
use std::ffi::CString;
use cgmath::{Matrix,Matrix4};

pub struct Program {
	pub vs: GLuint,
	pub fs: GLuint,
	pub id: GLuint,
}

impl Program {
	fn compile_shader(src: &str, ty: GLenum) -> GLuint {
		let shader;
		unsafe {
			shader = gl::CreateShader(ty);
			
			// Attempt to compile the shader
			let c_str = CString::new(src.as_bytes()).unwrap();
			gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
			gl::CompileShader(shader);

			// Get the compile status
			let mut status = gl::FALSE as GLint;
			gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

			// Fail on error
			if status != (gl::TRUE as GLint) {
				let mut len = 0;
				gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
				let mut buf = Vec::with_capacity(len as usize);
				buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
				gl::GetShaderInfoLog(shader,
									 len,
									 ptr::null_mut(),
									 buf.as_mut_ptr() as *mut GLchar);
				panic!("{}",
					   str::from_utf8(&buf)
						   .ok()
						   .expect("ShaderInfoLog not valid utf8"));
			}
		};
		shader
	}

	pub fn new(vs_text: &str, fs_text: &str) -> Program {
		let vs = Program::compile_shader(vs_text, gl::VERTEX_SHADER);
		let fs = Program::compile_shader(fs_text, gl::FRAGMENT_SHADER);
	
		unsafe {
			let program = gl::CreateProgram();
			gl::AttachShader(program, vs);
			gl::AttachShader(program, fs);
			gl::LinkProgram(program);
			
			// Get the link status
			let mut status = gl::FALSE as GLint;
			gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

			// Fail on error
			if status != (gl::TRUE as GLint) {
				let mut len: GLint = 0;
				gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
				
				let mut buf = Vec::with_capacity(len as usize);
				buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
				
				gl::GetProgramInfoLog(program,
									  len,
									  ptr::null_mut(),
									  buf.as_mut_ptr() as *mut GLchar);
									  
				panic!("{}",
					   str::from_utf8(&buf)
						   .ok()
						   .expect("ProgramInfoLog not valid utf8"));
			}
			
			Program { vs, fs, id: program }
		}
	}
	
	pub fn bind(&self) {
		unsafe {
			gl::UseProgram(self.id);
		};
	}
	
	pub fn bind_uniform_matrix4(&self, name: &str, value: &Matrix4<f32>) {
		unsafe {
			let loc = gl::GetUniformLocation(self.id, CString::new(name).unwrap().as_ptr());
			gl::UniformMatrix4fv(loc, 1, gl::FALSE, value.as_ptr());
		};
	}
	
	pub fn bind_uniform_int32(&self, name: &str, value: i32) {
		unsafe {
			let loc = gl::GetUniformLocation(self.id, CString::new(name).unwrap().as_ptr());
			gl::Uniform1i(loc, value);
		};
	}
}

impl Drop for Program {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteShader(self.fs);
			gl::DeleteShader(self.vs);
			gl::DeleteProgram(self.id);
		};
	}
}