extern crate byteorder;
extern crate cgmath;
extern crate gl;
extern crate glutin;
extern crate png;
mod mesh;
mod md2;
mod image;

use std::fs::File;
use std::path::Path;

fn main() {
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
	}
		
	let image = image::load_image(&Path::new("test.png")).unwrap();

	let mut file = File::open(&Path::new("test.md2")).unwrap();
	
	let mut mesh = md2::read_md2_model(&mut file, &image).unwrap();
	
	let proj = cgmath::perspective(
		cgmath::Deg(90.0),
		4.0/3.0,
		1.0,
		20.0);
		
	let mut running = true;

    while running {
        unsafe {
            // Clear the screen to black
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

		mesh.draw(&proj);

        window.swap_buffers().unwrap();

		events_loop.poll_events(|event| {
			match event {
				glutin::Event::WindowEvent { event: glutin::WindowEvent::Closed, .. } => {
					running = false;
				},
				_ => (),
			}
		});
    }
}
