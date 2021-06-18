extern crate glium;
extern crate image;

use glium::{
    glutin::{
        self,
        dpi::{PhysicalSize, Size},
        event::*,
        event_loop::{self, EventLoop},
        window::WindowBuilder,
        ContextBuilder, NotCurrent,
    },
    implement_vertex,
    index::{IndexBuffer, PrimitiveType},
    //index::NoIndices,
    texture::{RawImage2d, Texture2d},
    uniform,
    Display,
    Surface,
    VertexBuffer,
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

    // window and event loops
    let el: EventLoop<()> = glutin::event_loop::EventLoop::new();
    let wb: WindowBuilder = WindowBuilder::new()
        .with_title("Hello, World!")
        .with_resizable(false)
        .with_inner_size(Size::Physical(PhysicalSize::<u32>::new(1300, 700)));
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
        position: [-0.5, 0.5],
        tex_coords: [1.0, 0.0],
    };
    let v3: Vertex = Vertex {
        position: [0.5, -0.5],
        tex_coords: [0.0, 1.0],
    };
    let v4: Vertex = Vertex {
        position: [0.5, 0.5],
        tex_coords: [1.0, 1.0],
    };
    let shape: Vec<Vertex> = vec![v1, v2, v3, v4];

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

    // let index_buffer: NoIndices = NoIndices(glium::index::PrimitiveType::TrianglesList);
    let index_buffer = IndexBuffer::new(
        &display,
        PrimitiveType::TrianglesList,
        &vec![0 as u16, 1, 2, 1, 3, 2],
    )
    .unwrap();

    let mut pos_mods: [f32; 3] = [0.0, 0.0, 1.0];
    let mut pos_mod_speeds: [f32; 3] = [0.0, 0.0, 0.0];

    el.run(move |ev, _, control_flow| {
        let next_frame_time = Instant::now() + Duration::from_nanos(16_666_667);
        *control_flow = event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = event_loop::ControlFlow::Exit;
            }
            Event::DeviceEvent {
                event:
                    DeviceEvent::Key(KeyboardInput {
                        virtual_keycode: v_key_code,
                        state: e_state,
                        ..
                    }),
                ..
            } => match e_state {
                ElementState::Pressed => match v_key_code {
                    Some(VirtualKeyCode::W) => {
                        pos_mod_speeds[1] = 0.01;
                    }
                    Some(VirtualKeyCode::S) => {
                        pos_mod_speeds[1] = -0.01;
                    }
                    Some(VirtualKeyCode::Left)|
                    Some(VirtualKeyCode::A) => {
                        pos_mod_speeds[0] = -0.01;
                    }
                    Some(VirtualKeyCode::Right)|
                    Some(VirtualKeyCode::D) => {
                        pos_mod_speeds[0] = 0.01;
                    }
                    Some(VirtualKeyCode::Up) => {
                        pos_mod_speeds[2] = 0.01;
                    }
                    Some(VirtualKeyCode::Down) => {
                        pos_mod_speeds[2] = -0.01;
                    }
                    Some(VirtualKeyCode::R) => {
                        pos_mods = [0.0, 0.0, 1.0];
                    }
                    _ => {}
                },
                ElementState::Released => match v_key_code {
                    Some(VirtualKeyCode::W)|
                    Some(VirtualKeyCode::S) => {
                        pos_mod_speeds[1] = 0.0;
                    }
                    Some(VirtualKeyCode::Left)|
                    Some(VirtualKeyCode::Right)|
                    Some(VirtualKeyCode::A)|
                    Some(VirtualKeyCode::D) => {
                        pos_mod_speeds[0] = 0.0;
                    }
                    Some(VirtualKeyCode::Up)|
                    Some(VirtualKeyCode::Down) => {
                        pos_mod_speeds[2] = 0.0;
                    }
                    _ => {}
                }
            },
            _ => {}
        }

        pos_mods[0] += pos_mod_speeds[0];
        pos_mods[1] += pos_mod_speeds[1];
        pos_mods[2] += pos_mod_speeds[2];

        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, -1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [pos_mods[0], pos_mods[1], 0.0, pos_mods[2]],
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
