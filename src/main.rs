#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

use gfx::traits::FactoryExt;
use gfx::Device;

pub type SlpDataU16 = (gfx::format::R8_G8, gfx::format::Uint);
pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines!{
    vertex Vertex2d {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    // constant Locals {
    //     transform_matx: [[f32; 4]; 4] = "transform_matx",
    // }

    pipeline pipe {
        // transform_matx: gfx::Global<[[f32; 4]; 4]> = "transform_matx",
        // locals: gfx::ConstantBuffer<Locals> = "Locals",
        vbuf: gfx::VertexBuffer<Vertex2d> = (),
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

const VERTEX_SRC: &'static str = r#"
    #version 150 core

    in vec2 a_Pos;
    in vec3 a_Color;
    out vec4 v_Color;

    uniform Locals {
        mat4 transform_matx;
    };

    void main() {
        v_Color = vec4(a_Color, 1.0);
        gl_Position = /*transform_matx **/ vec4(a_Pos, 0.0, 1.0);
    }
"#;

const FRAGMENT_SRC: &'static str = r#"
    #version 150 core

    in vec4 v_Color;
    out vec4 Target0;

    void main() {
        Target0 = v_Color;
    }
"#;

const TRIANGLE: [Vertex2d; 3] = [Vertex2d {
                                     pos: [-0.5, -0.5],
                                     color: [1.0, 0.0, 0.0],
                                 },
                                 Vertex2d {
                                     pos: [0.5, -0.5],
                                     color: [0.0, 1.0, 0.0],
                                 },
                                 Vertex2d {
                                     pos: [0.0, 0.5],
                                     color: [0.0, 0.0, 1.0],
                                 }];

const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

pub fn main() {
    let builder = glutin::WindowBuilder::new()
        .with_title("gfx triangle".to_string())
        .with_dimensions(1024, 768)
        .with_vsync();

    let (window, mut device, mut factory, main_color, mut main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    let pso = factory
        .create_pipeline_simple(VERTEX_SRC.as_bytes(), FRAGMENT_SRC.as_bytes(), pipe::new())
        .unwrap();

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TRIANGLE, ());
    let mut data = pipe::Data {
        // transform_matx:  [[1.0, 0.0, 0.0, 0.0],
        //                   [0.0, 1.0, 0.0, 0.0],
        //                   [0.0, 0.0, 1.0, 0.0],
        //                   [0.0, 0.0, 0.0, 1.0f32]],
        // locals: factory.create_constant_buffer(1),
        vbuf: vertex_buffer,
        out: main_color,
    };

    'main: loop {
        for event in window.poll_events() {
            match event {
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
                glutin::Event::Closed => break 'main,
                glutin::Event::Resized(_width, _height) => {
                    gfx_window_glutin::update_views(&window, &mut data.out, &mut main_depth);
                }
                _ => {}
            }
        }

        encoder.clear(&data.out, CLEAR_COLOR);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
