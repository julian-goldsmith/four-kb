use gl;
use gl::types::*;
use std::mem;
use std::ptr;
use std::str;
use std::ffi::CString;

pub struct Mesh {
	pub program: GLuint,
	pub vs: GLuint,
	pub fs: GLuint,
	pub vao: GLuint,
	pub vbo: GLuint,
}

impl Mesh {
	pub fn new(vertex_shader: &str, fragment_shader: &str, vertex_data: &[GLfloat]) -> Mesh {
		let vs = compile_shader(vertex_shader, gl::VERTEX_SHADER);
		let fs = compile_shader(fragment_shader, gl::FRAGMENT_SHADER);
		let program = link_program(vs, fs);
		
		let (vao, vbo) = create_vertex_buffer(vertex_data, program);
		
		Mesh { program, vs, fs, vao, vbo }
	}
	
	pub fn draw(&self) {
		unsafe {
			gl::UseProgram(self.program);
			gl::BindVertexArray(self.vao);
			
			// Draw a triangle from the 3 vertices
			gl::DrawArrays(gl::TRIANGLES, 0, 3);
		}
	}
}

impl Drop for Mesh {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteProgram(self.program);
			gl::DeleteShader(self.fs);
			gl::DeleteShader(self.vs);
			gl::DeleteBuffers(1, &self.vbo);
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

fn create_vertex_buffer(vertex_data: &[GLfloat], program: GLuint) -> (GLuint, GLuint) {
    let mut vao = 0;
    let mut vbo = 0;

    unsafe {
        // Create Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (vertex_data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       mem::transmute(&vertex_data[0]),
                       gl::STATIC_DRAW);

        // Use shader program
        gl::UseProgram(program);
        gl::BindFragDataLocation(program, 0, CString::new("out_color").unwrap().as_ptr());

        // Specify the layout of the vertex data
        let pos_attr = gl::GetAttribLocation(program, CString::new("position").unwrap().as_ptr());
        gl::EnableVertexAttribArray(pos_attr as GLuint);
        gl::VertexAttribPointer(pos_attr as GLuint,
                                2,
                                gl::FLOAT,
                                gl::FALSE as GLboolean,
                                0,
                                ptr::null());
    };
	
	(vao, vbo)
}