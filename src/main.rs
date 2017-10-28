extern crate byteorder;
extern crate cgmath;
extern crate gl;
extern crate glutin;
extern crate png;
extern crate fbx_direct;
extern crate time;
mod mesh;
mod image;
mod gfx;
mod object;
mod model;
mod model_loader;

use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use object::Object;
use time::Duration;
use mesh::Mesh;
use cgmath::{Matrix4, Vector2, Vector3,Decomposed,Basis3,Deg,Rotation3};

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
	
	let mdl: Mesh = {
        let mut file = File::open(&Path::new("assets/monkey.mdl")).unwrap();
        let mdl = model_loader::load_model(&mut file);
        mdl.into()
	};

	unsafe {
		gl::Enable(gl::DEPTH_TEST);
		gl::Enable(gl::CULL_FACE);
		
		gl::ActiveTexture(gl::TEXTURE0);
		gl::ActiveTexture(gl::TEXTURE1);
		gl::ActiveTexture(gl::TEXTURE2);
	}

	//let mut object: Object = mdl.into();
	
	let mut proj = cgmath::PerspectiveFov {
		fovy: cgmath::Deg(90.0).into(),
		aspect: 4.0/3.0,
		near: 1.0,
		far: 20.0,
    };

	let mut running = true;

    let mut frames = 0;
    let mut duration = Duration::zero();

    let mut deg = 0.0;

    while running {
        let time1 = time::get_time();

        unsafe {
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let trans = Decomposed::<Vector3<f32>, Basis3<f32>> {
                    scale: 1.0,
                    rot: Basis3::from_angle_y(Deg(deg)) * Basis3::from_angle_x(Deg(-90.0)),
                    disp: Vector3::new(0.0, 0.0, -2.75),
                }.into();//read_transform(reader);
		//object.draw(proj);
        let mproj = proj.into();
        mdl.draw(&mproj, &trans);

        //deg += 2.0;

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

        let time2 = time::get_time();
        let frame_duration = time2 - time1;

        frames = frames + 1;
        duration = duration + frame_duration;

        if duration > Duration::seconds(1) {
            println!("{} FPS", frames);

            frames = 0;
            duration = Duration::zero();
        }
    }
}
