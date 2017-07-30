extern crate byteorder;
extern crate cgmath;
extern crate gl;
extern crate glutin;
extern crate png;
extern crate fbx_direct;
mod mesh;
mod image;
mod fbx;
mod gfx;
mod object;

use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use object::Object;

fn main() {
	let mdl = {
		let fbx = File::open(&Path::new("monkey.fbx")).unwrap();
		let buf = BufReader::new(fbx);
		fbx::read(buf)
	};

	let events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new().
		with_vsync().
		build(&events_loop).
		unwrap();

    // It is essential to make the context current before calling `gl::load_with`.
    unsafe { window.make_current() }.unwrap();

    // Load the OpenGL function pointers
    // TODO: `as *const _` will not be needed once glutin is updated to the latest gl version
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
	
	unsafe {
		gl::Enable(gl::DEPTH_TEST);
		gl::Enable(gl::CULL_FACE);
		
		gl::ActiveTexture(gl::TEXTURE0);
		gl::ActiveTexture(gl::TEXTURE1);
		gl::ActiveTexture(gl::TEXTURE2);
	}

	let mut object: Object = mdl.into();
	
	let mut proj = cgmath::PerspectiveFov {
		fovy: cgmath::Deg(90.0).into(),
		aspect: 4.0/3.0,
		near: 1.0,
		far: 20.0,
    };

	let mut running = true;

    while running {
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

		object.draw(proj);

        window.swap_buffers().unwrap();

		events_loop.poll_events(|event| {
			match event {
				glutin::Event::WindowEvent { event: glutin::WindowEvent::Closed, .. } => {
					running = false;
				},
                glutin::Event::WindowEvent { event: glutin::WindowEvent::Resized(width, height), .. } => {
                    proj = cgmath::PerspectiveFov {
                        fovy: cgmath::Deg(90.0).into(),
                        aspect: width as f32 / height as f32,
                        near: 1.0,
                        far: 20.0,
                    };
                },
				_ => (),
			}
		});
    }
}
