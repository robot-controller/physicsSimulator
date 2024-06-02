use std::sync::Arc;

use wgpu::util::DeviceExt;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.0868241, 0.49240386, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // A
    Vertex {
        position: [-0.49513406, 0.06958647, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // B
    Vertex {
        position: [-0.21918549, -0.44939706, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // C
    Vertex {
        position: [0.35966998, -0.3473291, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // D
    Vertex {
        position: [0.44147372, 0.2347359, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // E
];

const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];

#[derive(Default)]
struct WGPUApp<'a> {
    window: Option<Arc<Window>>,
    surface: Option<wgpu::Surface<'a>>,
    device: Option<wgpu::Device>,
    queue: Option<wgpu::Queue>,
    config: Option<wgpu::SurfaceConfiguration>,
    size: Option<winit::dpi::PhysicalSize<u32>>,
    render_pipeline: Option<wgpu::RenderPipeline>,
    vertex_buffer: Option<wgpu::Buffer>,
    num_indices: u32,
    index_buffer: Option<wgpu::Buffer>,
}

impl<'a> ApplicationHandler for WGPUApp<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
        let wgpu_instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            flags: wgpu::InstanceFlags::DEBUG,
            dx12_shader_compiler: wgpu::Dx12Compiler::Fxc,
            gles_minor_version: wgpu::Gles3MinorVersion::Automatic,
        });

        let available_gpus = wgpu_instance.enumerate_adapters(wgpu::Backends::PRIMARY);
        for adapter in available_gpus {
            println!("Available GPU: {:?}", adapter.get_info());
        }

        let surface = wgpu_instance.create_surface(window.clone()).unwrap();

        let size = window.inner_size();
        let adapter =
            pollster::block_on(wgpu_instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            }))
            .unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Good device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
            },
            None,
        ))
        .unwrap();

        let info = adapter.get_info();
        println!("Selected GPU info: {:?}", info);

        let surface_capabilities = surface.get_capabilities(&adapter);

        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .filter(|format| format.is_srgb())
            .next()
            .unwrap_or(surface_capabilities.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        self.window = Some(window);
        self.surface = Some(surface);
        self.device = Some(device);
        self.queue = Some(queue);
        self.config = Some(config);
        self.size = Some(size);
        self.render_pipeline = Some(render_pipeline);
        self.vertex_buffer = Some(vertex_buffer);
        self.num_indices = INDICES.len() as u32;
        self.index_buffer = Some(index_buffer);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::Resized(new_size) => {
                println!("The window was resized to {:?}", new_size);
                self.resize(new_size);
            }
            WindowEvent::RedrawRequested => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                self.update();
                match self.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => self.resize(self.size.unwrap()),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            _ => (),
        }
    }
}

impl<'a> WGPUApp<'a> {
    pub fn window(&self) -> &Window {
        &self.window.as_ref().unwrap()
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = Some(new_size);
            self.config.as_mut().unwrap().width = new_size.width;
            self.config.as_mut().unwrap().height = new_size.height;
            self.surface.as_ref().unwrap().configure(
                &self.device.as_ref().unwrap(),
                &self.config.as_ref().unwrap(),
            );
        }
    }

    fn input(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {
        // Maybe later
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.as_ref().unwrap().get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            self.device
                .as_ref()
                .unwrap()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 1.0,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(self.render_pipeline.as_ref().unwrap());
            render_pass.set_vertex_buffer(0, self.vertex_buffer.as_ref().unwrap().slice(..));
            render_pass.set_index_buffer(
                self.index_buffer.as_ref().unwrap().slice(..),
                wgpu::IndexFormat::Uint16,
            );
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        // submit will accept anything that implements IntoIter
        self.queue
            .as_ref()
            .unwrap()
            .submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

pub async fn run_wgpu_window() {
    let event_loop = EventLoop::new().unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = WGPUApp::default();

    let _ = event_loop.run_app(&mut app);
}
