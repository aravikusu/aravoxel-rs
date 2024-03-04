use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use crate::engine::assets::Assets;
use crate::engine::resource::instance::{Instance, InstanceRaw};

use crate::engine::resource::model::{DrawModel, Model, ModelVertex};
use crate::engine::resource::texture::Texture;
use crate::engine::util::{load_model, Vertex};
use crate::entity::camera::{Camera, CameraController};
use crate::scene::scene::Scene;

#[allow(dead_code)]
pub struct WgpuTutorial {
    assets: Assets,
    render_pipeline: wgpu::RenderPipeline,

    obj_model: Model,

    camera_controller: CameraController,
    camera_bind_group: wgpu::BindGroup,

    instances: Vec<Instance>,
    instance_buffer: wgpu::Buffer,
}

const NUM_INSTANCES_PER_ROW: u32 = 10;

impl Scene for WgpuTutorial {
    async fn new(
        device: &wgpu::Device,
        config:
        &wgpu::SurfaceConfiguration,
        queue: &wgpu::Queue,
    ) -> Box<Self> {
        let assets = Assets::new(device, config).await;
        let texture_bind_group_layout = Texture::bind_group_layout(device);
        
        let obj_model = load_model("cube.obj", device, queue, &texture_bind_group_layout).await.unwrap();

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
            device,
        );
        let camera_bind_group_layout = Camera::bind_group_layout(device);
        let camera_bind_group = camera_controller.camera.create_bind_group(&camera_bind_group_layout, device);

        // Instances - iterate through the amount we have, then create a buffer.
        let instances = (0..NUM_INSTANCES_PER_ROW).flat_map(|z| {
            (0..NUM_INSTANCES_PER_ROW).map(move |x| {
                let x = 3.0 * (x as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);
                let z = 3.0 * (z as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);

                let position = glam::Vec3 {
                    x,
                    y: 0.0,
                    z,
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
        let instance_buffer = InstanceRaw::create_buffer(&instances, device);

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
                    module: &assets.color_shader,
                    entry_point: "vs_main",
                    buffers: &[
                        ModelVertex::desc(),
                        InstanceRaw::desc(),
                    ],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &assets.color_shader,
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
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: Texture::DEPTH_FORMAT,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            });
        Box::from(Self {
            render_pipeline,
            camera_controller,
            instances,
            instance_buffer,
            obj_model,
            camera_bind_group,
            assets
        })
    }

    fn update(&mut self, queue: &wgpu::Queue) {
        self.camera_controller.update_camera();
        self.camera_controller.camera_uniform.update_view_proj(&self.camera_controller.camera);
        queue.write_buffer(&self.camera_controller.camera.buffer, 0, bytemuck::cast_slice(&[self.camera_controller.camera_uniform]))
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
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.assets.depth_texture.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.draw_model_instanced(&self.obj_model, 0..self.instances.len() as u32, &self.camera_bind_group);
    }

    fn input(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                self.camera_controller.keyboard_input(event);
            }
            _ => ()
        }
    }

    fn resize(&mut self, _new_size: PhysicalSize<u32>, device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) {
        self.assets.depth_texture = Texture::create_depth_texture(device, config);
    }
}