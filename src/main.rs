extern crate noise;

extern crate image;

#[macro_use(implement_vertex, uniform)]
extern crate glium;

use std::io::Cursor;
use std::time::SystemTime;
use std::fs;
use std::vec;

// use noise::{NoiseFn, Perlin};

// use noise::utils::PlaneMapBuilder;
// use crate::noise::utils::NoiseMapBuilder;

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

	const VERT_RES: usize = 10;

	let mut vertex_array = Vec::new();

	for i in 0..VERT_RES {
		for j in 0..VERT_RES {
			let fi = (i as f32) / (VERT_RES as f32);
			let fj = (j as f32) / (VERT_RES as f32);

			let vi = 2.0 * fi - 1.0;
			let vj = 2.0 * fj - 1.0;

			let ti = fi;
			let tj = fj;

			let vertex = Vertex { position: [vi, vj], tex_coords: [ti, tj] };
			vertex_array.push(vertex);
		}
	}

	// const NUM_TRIANGLES = 2 * VERT_RES * VERT_RES;


	// // create the vertex and index buffer
	// let vertex_buffer = glium::VertexBuffer::new(&display, &vertex_array).unwrap();
	// let index_buffer = glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList,
	// 	&[0u16,1,2,2,3,0]).unwrap();

	// load the image
	let image = image::load(Cursor::new(&include_bytes!("../assets/cherry_blossom.png")),
							image::ImageFormat::Png).unwrap().to_rgba8();
	let image_dimensions = image.dimensions();
	let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

	let texture = glium::texture::Texture2d::new(&display, image).unwrap();

	// vertex and index buffer
	let vertex_buffer = glium::VertexBuffer::new(&display, &vertex_array).unwrap();
	let index_buffer = glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList,
		&vec![0u16, 1, 10, 1, 2, 11, 2, 3, 12, 3, 4, 13, 4, 5, 14, 5, 6, 15, 6, 7, 16, 7, 8, 17, 8, 9, 18, 10, 11, 20, 11, 12, 21, 12, 13, 22, 13, 14, 23, 14, 15, 24, 15, 16, 25, 16, 17, 26, 17, 18, 27, 18, 19, 28, 20, 21, 30, 21, 22, 31, 22, 23, 32, 23, 24, 33, 24, 25, 34, 25, 26, 35, 26, 27, 36, 27, 28, 37, 28, 29, 38, 30, 31, 40, 31, 32, 41, 32, 33, 42, 33, 34, 43, 34, 35, 44, 35, 36, 45, 36, 37, 46, 37, 38, 47, 38, 39, 48, 40, 41, 50, 41, 42, 51, 42, 43, 52, 43, 44, 53, 44, 45, 54, 45, 46, 55, 46, 47, 56, 47, 48, 57, 48, 49, 58, 50, 51, 60, 51, 52, 61, 52, 53, 62, 53, 54, 63, 54, 55, 64, 55, 56, 65, 56, 57, 66, 57, 58, 67, 58, 59, 68, 60, 61, 70, 61, 62, 71, 62, 63, 72, 63, 64, 73, 64, 65, 74, 65, 66, 75, 66, 67, 76, 67, 68, 77, 68, 69, 78, 70, 71, 80, 71, 72, 81, 72, 73, 82, 73, 74, 83, 74, 75, 84, 75, 76, 85, 76, 77, 86, 77, 78, 87, 78, 79, 88, 80, 81, 90, 81, 82, 91, 82, 83, 92, 83, 84, 93, 84, 85, 94, 85, 86, 95, 86, 87, 96, 87, 88, 97, 88, 89, 98, 1, 10, 11, 2, 11, 12, 3, 12, 13, 4, 13, 14, 5, 14, 15, 6, 15, 16, 7, 16, 17, 8, 17, 18, 9, 18, 19, 11, 20, 21, 12, 21, 22, 13, 22, 23, 14, 23, 24, 15, 24, 25, 16, 25, 26, 17, 26, 27, 18, 27, 28, 19, 28, 29, 21, 30, 31, 22, 31, 32, 23, 32, 33, 24, 33, 34, 25, 34, 35, 26, 35, 36, 27, 36, 37, 28, 37, 38, 29, 38, 39, 31, 40, 41, 32, 41, 42, 33, 42, 43, 34, 43, 44, 35, 44, 45, 36, 45, 46, 37, 46, 47, 38, 47, 48, 39, 48, 49, 41, 50, 51, 42, 51, 52, 43, 52, 53, 44, 53, 54, 45, 54, 55, 46, 55, 56, 47, 56, 57, 48, 57, 58, 49, 58, 59, 51, 60, 61, 52, 61, 62, 53, 62, 63, 54, 63, 64, 55, 64, 65, 56, 65, 66, 57, 66, 67, 58, 67, 68, 59, 68, 69, 61, 70, 71, 62, 71, 72, 63, 72, 73, 64, 73, 74, 65, 74, 75, 66, 75, 76, 67, 76, 77, 68, 77, 78, 69, 78, 79, 71, 80, 81, 72, 81, 82, 73, 82, 83, 74, 83, 84, 75, 84, 85, 76, 85, 86, 77, 86, 87, 78, 87, 88, 79, 88, 89, 81, 90, 91, 82, 91, 92, 83, 92, 93, 84, 93, 94, 85, 94, 95, 86, 95, 96, 87, 96, 97, 88, 97, 98, 89, 98, 99]
		).unwrap();
		// end of texture stuff

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