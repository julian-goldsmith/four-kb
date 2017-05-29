extern crate byteorder;
extern crate gl;
extern crate glutin;
mod mesh;
mod md2;

use gl::types::*;
use mesh::Mesh;
use std::str;

// Vertex data
static VERTEX_DATA: [GLfloat; 6] = [0.0, 0.5, 0.5, -0.5, -0.5, -0.5];

// Shader sources
static VS_SRC: &'static str = "#version 150\n\
    in vec2 position;\n\
    void main() {\n\
       gl_Position = vec4(position, 0.0, 1.0);\n\
    }";

static FS_SRC: &'static str = "#version 150\n\
    out vec4 out_color;\n\
    void main() {\n\
       out_color = vec4(1.0, 1.0, 1.0, 1.0);\n\
    }";

// Vertex data
static VERTEX_DATA2: [GLfloat; 6] = [0.0, 0.5, 0.0, -0.5, -0.5, -1.0];

// Shader sources
static VS2_SRC: &'static str = "#version 150\n\
    in vec2 position;\n\
    void main() {\n\
       gl_Position = vec4(position, 0.0, 1.0);\n\
    }";

static FS2_SRC: &'static str = "#version 150\n\
    out vec4 out_color;\n\
    void main() {\n\
       out_color = vec4(1.0, 0, 0, 1.0);\n\
    }";

fn main() {
    let window = glutin::Window::new().unwrap();

    // It is essential to make the context current before calling `gl::load_with`.
    unsafe { window.make_current() }.unwrap();

    // Load the OpenGL function pointers
    // TODO: `as *const _` will not be needed once glutin is updated to the latest gl version
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

	let mesh = Mesh::new(VS_SRC, FS_SRC, &VERTEX_DATA);
	let mesh2 = Mesh::new(VS2_SRC, FS2_SRC, &VERTEX_DATA2);

    for event in window.wait_events() {
        unsafe {
            // Clear the screen to black
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

			mesh.draw();
			mesh2.draw();
        }

        window.swap_buffers().unwrap();

        if let glutin::Event::Closed = event {
            break;
        }
    }
}
