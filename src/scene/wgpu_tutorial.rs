use wgpu::util::DeviceExt;
use winit::event::WindowEvent;

use crate::engine::resource::texture::Texture;
use crate::engine::resource_manager::ResourceManager;
use crate::engine::util::{Instance, InstanceRaw, Vertex};
use crate::entity::camera::CameraController;
use crate::mesh::mesh::Mesh;
use crate::scene::scene::Scene;

pub struct WgpuTutorial {
    render_pipeline: wgpu::RenderPipeline,
    mesh: Mesh,
    diffuse_bind_group: wgpu::BindGroup,
    resource_manager: ResourceManager,

    camera_controller: CameraController,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,

    instances: Vec<Instance>,
    instance_buffer: wgpu::Buffer,
}

const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.0868241, 0.49240386, 0.0], tex_coords: [0.4131759, 0.00759614] }, // A
    Vertex { position: [-0.49513406, 0.06958647, 0.0], tex_coords: [0.0048659444, 0.43041354] }, // B
    Vertex { position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.28081453, 0.949397] }, // C
    Vertex { position: [0.35966998, -0.3473291, 0.0], tex_coords: [0.85967, 0.84732914] }, // D
    Vertex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 0.2652641] }, // E
];


const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
];

const NUM_INSTANCS_PER_ROW: u32 = 10;
const INSTANCE_DISPLACEMENT: glam::Vec3 = glam::Vec3::new(NUM_INSTANCS_PER_ROW as f32 * 0.5, 0.0, NUM_INSTANCS_PER_ROW as f32 * 0.5);

impl Scene for WgpuTutorial {
    fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, queue: &wgpu::Queue) -> Box<Self> {
        let mesh = Mesh::new(VERTICES, INDICES, &device);

        let mut resource_manager = ResourceManager::new();
        let diffuse_texture = resource_manager.load_texture(&device, &queue, "happy-tree.png", "happytree".to_string());

        let texture_bind_group_layout = Texture::bind_group_layout(&device);
        let diffuse_bind_group = diffuse_texture.create_bind_group(&texture_bind_group_layout, &device);

        // Camera - more refactor needed
        let camera_controller = CameraController::new(
            0.2,
            (0.0, 1.0, 2.0).into(),
            (0.0, 0.0, 0.0).into(),
            glam::Vec3::Y,
            config.width as f32 / config.height as f32,
            45.0,
            0.1,
            100.0,
        );

        let camera_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera buffer"),
                contents: bytemuck::cast_slice(&[camera_controller.camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let camera_bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("Camera bind group layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            }
        );

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera bind group"),
            layout: &camera_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                }
            ],
        });

        // Instances - refactor into its own thing
        let instances = (0..NUM_INSTANCS_PER_ROW).flat_map(|z| {
            (0..NUM_INSTANCS_PER_ROW).map(move |x| {
                let position = glam::Vec3 {
                    x: x as f32,
                    y: 0.0,
                    z: z as f32,
                };

                let rotation = if position.is_nan() {
                    // Needed so an object at (0, 0, 0) doesn't get scaled to 0
                    // since Quaternions can effect scale if they're not "correct"
                    glam::Quat::from_axis_angle(glam::Vec3::Z, 0.0)
                } else {
                    glam::Quat::from_axis_angle(position.normalize(), 45.0)
                };

                Instance { position, rotation }
            })
        }).collect::<Vec<_>>();

        let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        let instance_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX
            }
        );

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("WgpuTutorial Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("WgpuTutorial Pipeline Layout"),
                bind_group_layouts: &[
                    &texture_bind_group_layout,
                    &camera_bind_group_layout
                ],
                push_constant_ranges: &[],
            });

        let render_pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("WgpuTutorial Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[
                        Vertex::desc(),
                        InstanceRaw::desc(),
                    ],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
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
                    // Setting this to another other than FIll requires Features::NON_FILL_POLYGON_MODE
                    polygon_mode: wgpu::PolygonMode::Fill,
                    // Requires Features::DEPTH_CLIP_CONTROL,
                    unclipped_depth: false,
                    // Requires Features::CONSERVATIVE_RASTERIZATION,
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
        Box::from(Self {
            render_pipeline,
            mesh,
            diffuse_bind_group,
            resource_manager,
            camera_controller,
            camera_buffer,
            camera_bind_group,
            instances,
            instance_buffer
        })
    }

    fn update(&mut self, queue: &wgpu::Queue) {
        self.camera_controller.update_camera();
        self.camera_controller.camera_uniform.update_view_proj(&self.camera_controller.camera);
        queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[self.camera_controller.camera_uniform]))
    }

    fn render(&mut self, view: &wgpu::TextureView, encoder: &mut wgpu::CommandEncoder) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[
                // @location(0) in fragment shader
                Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })
            ],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
        render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.mesh.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        render_pass.set_index_buffer(self.mesh.index_buffer.as_ref().unwrap().slice(..), wgpu::IndexFormat::Uint16);

        render_pass.draw_indexed(0..self.mesh.amount, 0, 0..self.instances.len() as _);
    }

    fn input(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                self.camera_controller.keyboard_input(&event);
            }
            _ => ()
        }
    }
}