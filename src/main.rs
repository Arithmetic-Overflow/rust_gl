extern crate noise;

extern crate image;

#[macro_use(implement_vertex, uniform)]
extern crate glium;

use std::io::Cursor;
use std::time::SystemTime;
use std::fs;

use noise::{NoiseFn, Perlin};

use glium::{glutin, Surface};
use glium::index::PrimitiveType;

#[derive(Copy, Clone)]
struct Vertex {
	position: [f32; 2],
	tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);    

fn main() {
	// keep track of the time
	let start_time = SystemTime::now();

	let event_loop = glutin::event_loop::EventLoop::new();
	let wb = glutin::window::WindowBuilder::new();
	let cb = glutin::ContextBuilder::new();
	let display = glium::Display::new(wb, cb, &event_loop).unwrap();

	// initialize the shaders
	let vert_shader_path = "shaders/texture.vert";
	let frag_shader_path = "shaders/texture.frag";

	let vert_shader_src = fs::read_to_string(vert_shader_path)
		.expect("Something went wrong reading the file");

	let frag_shader_src = fs::read_to_string(frag_shader_path)
		.expect("Something went wrong reading the file");

	// initialize the program
	let program = glium::Program::from_source(&display, &vert_shader_src, &frag_shader_src, None).unwrap();

	// // set up vertices
	let vertex1 = Vertex { position: [-1.0,-1.0], tex_coords: [0.0, 0.0] };
	let vertex2 = Vertex { position: [-1.0, 1.0], tex_coords: [0.0, 1.0] };
	let vertex3 = Vertex { position: [ 1.0, 1.0], tex_coords: [1.0, 1.0] };
	let vertex4 = Vertex { position: [ 1.0,-1.0], tex_coords: [1.0, 0.0] };
	let shape = vec![vertex1, vertex2, vertex3, vertex4];

	// // create the vertex and index buffer
	// let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
	// let index_buffer = glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList,
	// 	&[0u16,1,2,2,3,0]).unwrap();

	// load the image
	let image = image::load(Cursor::new(&include_bytes!("../assets/cherry_blossom.png")),
	                        image::ImageFormat::Png).unwrap().to_rgba8();
	let image_dimensions = image.dimensions();
	let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

	let texture = glium::texture::Texture2d::new(&display, image).unwrap();

	// texture stuff
	// let vertex1 = Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] };
	// let vertex2 = Vertex { position: [ 0.0,  0.5], tex_coords: [0.0, 1.0] };
	// let vertex3 = Vertex { position: [ 0.5, -0.25], tex_coords: [1.0, 0.0] };
	// let shape = vec![vertex1, vertex2, vertex3];

	let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
	let index_buffer = glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList,
		&[0u16,1,2,2,3,0]).unwrap();
		// end of texture stuff

	// noise generation
	let perlin = Perlin::new();

	// event loop
	event_loop.run(move |ev, _, control_flow| {
		let next_frame_time = std::time::Instant::now() +
			std::time::Duration::from_nanos(16_666_667);

		*control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
		match ev {
			glutin::event::Event::WindowEvent { event, .. } => match event {
				glutin::event::WindowEvent::CloseRequested => {
					*control_flow = glutin::event_loop::ControlFlow::Exit;
					return;
				},
				_ => return,
			},
			_ => (),
		}
		let time = 0.001*(start_time.elapsed().unwrap().as_millis() as f32);

		let mut noise: [[f32; 100]; 100];
		// for(let i = 0; i < 100; i++) {
		// 	for(let j = 0; j < 100; j++) {
		// 		noise[i][j] = 10.0;
		// 	}
		// }

		let uniforms = uniform! {
	    	matrix: [
		        [1.0, 0.0, 0.0, 0.0],
		        [0.0, 1.0, 0.0, 0.0],
		        [0.0, 0.0, 1.0, 0.0],
		        [0.0, 0.0, 0.0, 1.0f32],
		    ],
		    tex: &texture,
		    time: time,
		};

		let mut target = display.draw();
		target.clear_color(0.0, 0.0, 1.0, 1.0);

		target.draw(&vertex_buffer, &index_buffer, &program, &uniforms,
			&Default::default()).unwrap();

		target.finish().unwrap();
	});
}