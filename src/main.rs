use std::time::SystemTime;

use std::fs;

#[macro_use(implement_vertex, uniform)]
extern crate glium;

use glium::index::PrimitiveType;

#[derive(Copy, Clone)]
struct Vertex {
	position: [f32; 2],
}

implement_vertex!(Vertex, position);    

fn main() {
	use glium::{glutin, Surface};

	let start_time = SystemTime::now();

	let event_loop = glutin::event_loop::EventLoop::new();
	let wb = glutin::window::WindowBuilder::new();
	let cb = glutin::ContextBuilder::new();
	let display = glium::Display::new(wb, cb, &event_loop).unwrap();

	let vert_shader_path = "shaders/shader.vert";
	let frag_shader_path = "shaders/shader.frag";

	let vert_shader_src = fs::read_to_string(vert_shader_path)
		.expect("Something went wrong reading the file");

	let frag_shader_src = fs::read_to_string(frag_shader_path)
		.expect("Something went wrong reading the file");

	let program = glium::Program::from_source(&display, &vert_shader_src, &frag_shader_src, None).unwrap();

	let vertex1 = Vertex { position: [-1.0,-1.0] };
	let vertex2 = Vertex { position: [-1.0, 1.0] };
	let vertex3 = Vertex { position: [ 1.0, 1.0] };
	let vertex4 = Vertex { position: [ 1.0,-1.0] };
	let shape = vec![vertex1, vertex2, vertex3, vertex4];

	let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

	let index_buffer = glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList,
		&[0u16,1,2,2,3,0]).unwrap();

	event_loop.run(move |ev, _, control_flow| {
		let time = 0.0001*(start_time.elapsed().unwrap().as_millis() as f32);

		let mut target = display.draw();
		target.clear_color(0.0, 0.0, 1.0, 1.0);

		target.draw(&vertex_buffer, &index_buffer, &program, &uniform!{ time: time },
			&Default::default()).unwrap();

		target.finish().unwrap();

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
	});
}