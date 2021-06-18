extern crate glium;
extern crate image;

use glium::{
    glutin::{
        self,
        event::*,
        event_loop::{self, EventLoop},
        window::WindowBuilder,
        ContextBuilder, NotCurrent,
    },
    implement_vertex,
    index::NoIndices,
    texture::{RawImage2d, Texture2d},
    uniform, Display, Surface, VertexBuffer,
};
use std::{
    fs::read,
    io::Cursor,
    path::Path,
    time::{Duration, Instant},
};

pub fn read_file_as_string(pt: &Path) -> Result<String, &'static str> {
    let contents: String = String::from_utf8(read(pt).unwrap()).unwrap();
    Ok(contents)
}

#[derive(Debug, Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

fn main() {
    implement_vertex!(Vertex, position, tex_coords);
    let mut frame: f32 = -0.5;

    // window and event loops
    let el: EventLoop<()> = glutin::event_loop::EventLoop::new();
    let wb: WindowBuilder = WindowBuilder::new().with_title("Hello, World!");
    let cb: ContextBuilder<NotCurrent> = ContextBuilder::new();
    let display: Display = Display::new(wb, cb, &el).unwrap();

    // Tetures
    let wink_raw_rgba8 = image::load(
        Cursor::new(&include_bytes!("../assets/tex/wink-side.png")),
        image::ImageFormat::Png,
    )
    .unwrap()
    .to_rgba8();
    let wink_dimensions = wink_raw_rgba8.dimensions();
    let wink = RawImage2d::from_raw_rgba_reversed(&wink_raw_rgba8.into_raw(), wink_dimensions);
    let wink_texture: Texture2d = Texture2d::new(&display, wink).unwrap();
    println!("{:?}", wink_dimensions);
    println!("{}, {}", wink_dimensions.0 / 10, wink_dimensions.1 / 10);

    // shaders
    let vertex_shader_src: String =
        read_file_as_string(Path::new("./assets/shaders/default.vert")).unwrap();
    let fragment_shader_src: String =
        read_file_as_string(Path::new("./assets/shaders/default.frag")).unwrap();

    // triangle
    let v1: Vertex = Vertex {
        position: [-0.5, -0.5],
        tex_coords: [0.0, 0.0],
    };
    let v2: Vertex = Vertex {
        position: [0.0, 0.5],
        tex_coords: [1.0, 0.0],
    };
    let v3: Vertex = Vertex {
        position: [0.5, -0.5],
        tex_coords: [0.0, 1.0],
    };
    let shape: Vec<Vertex> = vec![v1, v2, v3];

    // upload to vertex buffer
    let vertex_buffer: VertexBuffer<Vertex> = VertexBuffer::new(&display, &shape).unwrap();

    // create program
    let program = glium::Program::from_source(
        &display,
        vertex_shader_src.as_str(),
        fragment_shader_src.as_str(),
        None,
    )
    .unwrap();

    let index_buffer: NoIndices = NoIndices(glium::index::PrimitiveType::TrianglesList);

    el.run(move |ev, _, control_flow| {
        let next_frame_time = Instant::now() + Duration::from_nanos(16_666_667);
        *control_flow = event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            _ => (),
        }

        frame += 0.0002;

        if frame > 0.7 {
            frame = -0.5;
        }

        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [frame, 0.0, 0.0, 1.0f32],
            ],
            tex: &wink_texture
        };

        // start drawing
        let mut target = display.draw();

        // rendering code
        target.clear_color(0.0, 0.34, 0.17, 1.0);
        target
            .draw(
                &vertex_buffer,
                &index_buffer,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();

        // finish drawing
        target.finish().unwrap();
    });
}
