use wgpu::*;
use winit::{
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use pollster::block_on;

const VERTICES: &[f32] = &[
    -0.0868241, 0.492284, 0.0, 1.0,
    0.414214, 0.141237, 0.0, 1.0,
    0.414214, -0.141237, 0.0, 1.0,
];

const INDICES: &[u16] = &[0, 1, 2];

fn main() {
    block_on(async {
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });

        let surface = unsafe { instance.create_surface(&winit::window::Window::new(&winit::window::WindowDescriptor::default())) };

        let adapter = instance
            .request_adapter(
                &RequestAdapterOptions {
                    power_preference: PowerPreference::default(),
                    force_fallback_adapter: false,
                    compatible_surface: Some(&surface),
                },
            )
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    features: Features::empty(),
                    limits: Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: 800,
            height: 600,
            present_mode: PresentMode::Fifo,
        };

        surface.configure(&device, &config);

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: BufferUsages::INDEX,
        });

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Shader"),
            source: ShaderSource::Wgsl(Cow::Borrowed(
                r#"
                    struct VertexInput {
                        [[location(0)]] position: vec4<f32>,
                    }

                    struct VertexOutput {
                        [[builtin(position)]] position: vec4<f32>,
                    }

                    [[stage(vertex)]]
                    fn vs_main(input: VertexInput) -> VertexOutput {
                        var output: VertexOutput;
                        output.position = input.position;
                        return output;
                    }

                    [[stage(fragment)]]
                    fn fs_main() -> [[location(0)]] vec4<f32> {
                        return vec4<f32>(1.0, 0.0, 0.0, 1.0);
                    }
                "#,
            )),
        });

        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: ProgrammableStageDescriptor {
                module: &shader,
                entry_point: "vs_main",
            },
            fragment: Some(ProgrammableStageDescriptor {
                module: &shader,
                entry_point: "fs_main",
            }),
            rasterization_state: Some(RasterizationStateDescriptor {
                front_face: FrontFace::Cw,
                cull_mode: None,
                ..Default::default()
            }),
            primitive_topology: PrimitiveTopology::TriangleList,
            color_states: &[ColorStateDescriptor {
                format: config.format,
                color_blend: BlendDescriptor {
                    src_factor: BlendFactor::SrcAlpha,
                    dst_factor: BlendFactor::OneMinusSrcAlpha,
                    operation: BlendOperation::Add,
                },
                alpha_blend: BlendDescriptor {
                    src_factor: BlendFactor::One,
                    dst_factor: BlendFactor::Zero,
                    operation: BlendOperation::Add,
                },
                write_mask: ColorWriteMask::ALL,
            }],
             
