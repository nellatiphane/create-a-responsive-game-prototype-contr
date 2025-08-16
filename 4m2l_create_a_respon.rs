// 4m2l_create_a_respon.rs

// Importing necessary libraries
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

use gfx::{
    Device,
    Factory,
    Format,
    Handle,
    Resources,
    encoder::{Encoder, Bundle},
    command::{CommandBuffer, ClearData},
    pso::PipelineState,
};
use gfx_window_glutin::{WindowBuilder, WindowTargets};

// Defining constants for the game
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const TITLE: &str = "4m2l Create a Responsive Game Prototype Controller";

// Defining a Game struct to hold the game state
struct Game {
    device: gfx::Device,
    factory: gfx::Factory,
    encoder: gfx::encoder::Encoder<gfx_window_glutin::WindowTargets>,
    bundle: gfx::encoder::Bundle<gfx_window_glutin::WindowTargets>,
    pipeline: gfx::pso::PipelineState<()>,
    vertex_buffer: gfx::Handle<[Vertex]>,
    instance_buffer: gfx::Handle<[Instance]>,
}

// Defining Vertex and Instance structs for the game
struct Vertex {
    pos: [f32; 2],
    uv: [f32; 2],
}

struct Instance {
    transform: [[f32; 4]; 4],
}

// Implementing the Game struct
impl Game {
    fn new() -> Self {
        // Creating a window and a device
        let window = WindowBuilder::new()
            .with_title(TITLE)
            .with_inner_size(WIDTH, HEIGHT)
            .build()
            .unwrap();
        let (mut device, mut factory, mut encoder, bundle) = window.device().clone();

        // Creating a vertex buffer
        let vertex_data: &[Vertex] = &[
            Vertex { pos: [-0.5, -0.5], uv: [0.0, 0.0] },
            Vertex { pos: [0.5, -0.5], uv: [1.0, 0.0] },
            Vertex { pos: [0.0, 0.5], uv: [0.5, 1.0] },
        ];
        let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(vertex_data, ());

        // Creating an instance buffer
        let instance_data: &[Instance] = &[
            Instance {
                transform: [
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ],
            },
        ];
        let (instance_buffer, slice) = factory.create_vertex_buffer_with_slice(instance_data, ());

        // Creating a pipeline
        let pipeline = factory.create_pipeline_simple(
            include_bytes!(" shader.bin "),
            gfx::PrimitiveTopology::TriangleList,
            (gfx::state::Rasterizer {
                polygon_mode: gfx::state::PolygonMode::Fill,
                .. gfx::state::Rasterizer::new Fill
            },),
            Some(gfx::state::Blend {
                targets: vec![gfx::state::BlendTarget {
                    mask: !0,
                    color: (0.0, 0.0, 0.0, 0.0),
                    alpha: (0.0, 0.0, 0.0, 0.0),
                }],
            }),
            gfx::state::Depth {
                fun: gfx::state::Comparison::LessEqual,
                write: true,
            },
        ).unwrap();

        Game {
            device,
            factory,
            encoder,
            bundle,
            pipeline,
            vertex_buffer,
            instance_buffer,
        }
    }

    fn run(&mut self) {
        // Running the game loop
        loop {
            // Encoding a command buffer
            let mut encoder = gfx::encoder::Encoder::new(self.device.clone(), gfx_window_glutin::WindowTargets::new(self.bundle.clone()));

            encoder.clear(&gfx_window_glutin::WindowTargets::new(self.bundle.clone()), ClearData {
                color: [0.1, 0.2, 0.3, 1.0],
                depth: 1.0,
                stencil: 0,
            });

            encoder.draw(&gfx_window_glutin::WindowTargets::new(self.bundle.clone()), &self.pipeline, &self.vertex_buffer, &self.instance_buffer, ());

            // Submitting and presenting
            let mut encoder = encoder.finish();
            self.device.submit(Some(encoder.clone()), &mut self.bundle);
            self.bundle.present();
        }
    }
}

fn main() {
    // Creating and running the game
    let mut game = Game::new();
    game.run();
}