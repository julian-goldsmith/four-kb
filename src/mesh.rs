use gl;
use gl::types::*;
use std::mem;
use std::ptr;
use std::str;
use std::ffi::CString;
use cgmath;
use cgmath::prelude::*;
use cgmath::{Matrix4, Vector3, Matrix, Deg, Basis3};
use image::Image;

pub struct Mesh {
	pub program: GLuint,
	pub vs: GLuint,
	pub fs: GLuint,
	pub vao: GLuint,
	pub vbo_verts: GLuint,
	pub vbo_texcoords: GLuint,
	pub tex: GLuint,
	pub num_verts: u32,
	
	pub transform: cgmath::Decomposed<Vector3<GLfloat>, Basis3<GLfloat>>,
}

impl Mesh {
	pub fn new(vertex_shader: &str, fragment_shader: &str, 
			   vertex_data: &[GLfloat], 
			   texcoord_data: &[GLfloat], 
			   image: &Image) -> Mesh {
		let vs = compile_shader(vertex_shader, gl::VERTEX_SHADER);
		let fs = compile_shader(fragment_shader, gl::FRAGMENT_SHADER);
		let program = link_program(vs, fs);
		
		let tex = create_texture(image);
	
		let vbo_verts = create_vbo(vertex_data);
		let vbo_texcoords = create_vbo(texcoord_data);
		
		let vao = create_vao(vbo_verts, vbo_texcoords, program);
		
		let transform = cgmath::Decomposed::<Vector3<GLfloat>, Basis3<GLfloat>> {
			scale: 0.05,
			rot: Basis3::from_angle_x(Deg(-90.0)),
			disp: Vector3::new(0.0, 0.0, -1.75),
		};
		
		Mesh { program, vs, fs, vao, vbo_verts, vbo_texcoords, tex, num_verts: vertex_data.len() as u32, transform }
	}
	
	pub fn draw(&mut self, proj: &Matrix4<GLfloat>) {
		let trans: Matrix4<GLfloat> = self.transform.clone().into();
		
		unsafe {
			gl::UseProgram(self.program);
			gl::BindVertexArray(self.vao);
			gl::BindTexture(gl::TEXTURE_2D, self.tex);
			
			let uni_trans = gl::GetUniformLocation(self.program, CString::new("trans").unwrap().as_ptr());
			gl::UniformMatrix4fv(uni_trans, 1, gl::FALSE, trans.as_ptr());
			
			let uni_proj = gl::GetUniformLocation(self.program, CString::new("proj").unwrap().as_ptr());
			gl::UniformMatrix4fv(uni_proj, 1, gl::FALSE, proj.as_ptr());
			
			let tex_loc = gl::GetUniformLocation(self.program, CString::new("tex").unwrap().as_ptr());
			gl::Uniform1i(tex_loc, 0);
			
			gl::DrawArrays(gl::TRIANGLES, 0, self.num_verts as i32);
		};
		
		self.transform.rot = self.transform.rot * Basis3::from_angle_z(Deg(0.25));
	}
}

impl Drop for Mesh {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteProgram(self.program);
			gl::DeleteShader(self.fs);
			gl::DeleteShader(self.vs);
			gl::DeleteBuffers(1, &self.vbo_verts);
			gl::DeleteBuffers(1, &self.vbo_texcoords);
			gl::DeleteVertexArrays(1, &self.vao);
		}
	}
}

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

fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
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
        program
    }
}

fn create_vao(vbo_verts: GLuint, vbo_texcoords: GLuint, program: GLuint) -> GLuint {
    let mut vao = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
    };
	
	bind_attribute("position", vbo_verts, 3, program);
	bind_attribute("texcoord", vbo_texcoords, 2, program);
	
	set_frag_data_name("out_color", program);
	
	vao
}

fn set_frag_data_name(name: &str, program: GLuint) {
    unsafe {
        gl::BindFragDataLocation(program, 0, CString::new(name).unwrap().as_ptr());
    };
}

fn bind_attribute(name: &str, vbo: GLuint, num_components: u16, program: GLuint) {
	unsafe {
        let attr = gl::GetAttribLocation(program, CString::new(name).unwrap().as_ptr());
        gl::EnableVertexAttribArray(attr as GLuint);
		
		gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
		gl::VertexAttribPointer(attr as GLuint,
								num_components as i32,
								gl::FLOAT,
								gl::FALSE as GLboolean,
								0,
								ptr::null());
	};
}

fn create_vbo(data: &[GLfloat]) -> GLuint {
	let mut vbo = 0;
	
	unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       mem::transmute(&data[0]),
                       gl::STATIC_DRAW);
	};
	
	vbo
}

fn create_texture(image: &Image) -> GLuint {
	let mut tex = 0;
	
	unsafe {
		gl::GenTextures(1, &mut tex);
		gl::BindTexture(gl::TEXTURE_2D, tex);
		
		let slice = &image.data[0..];
		gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, image.width as i32, image.height as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, mem::transmute(&slice[0]));
		
		gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
		gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
	};
	
	tex
}