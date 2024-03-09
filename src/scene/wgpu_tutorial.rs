use std::time::Duration;
use crate::engine::resource::instance::{Instance, InstanceRaw};
use crate::engine::resource::light::Light;
use winit::dpi::PhysicalSize;
use winit::event::{WindowEvent, MouseButton, ElementState, DeviceEvent};

use crate::engine::resource::model::{DrawLight, DrawModel, Model, ModelVertex};
use crate::engine::resource::texture::Texture;
use crate::engine::resource_manager::ResourceManager;
use crate::engine::util::{create_render_pipeline, load_model, Vertex};
use crate::entity::camera::{Camera, CameraController};
use crate::scene::scene::Scene;

#[allow(dead_code)]
pub struct WgpuTutorial {
    resource_manager: ResourceManager,
    render_pipeline: wgpu::RenderPipeline,
    light_render_pipeline: wgpu::RenderPipeline,

    obj_model: Model,

    camera_controller: CameraController,
    camera_bind_group: wgpu::BindGroup,

    instances: Vec<Instance>,
    instance_buffer: wgpu::Buffer,

    light: Light,
    light_bind_group: wgpu::BindGroup,

    mouse_pressed: bool,
}

const NUM_INSTANCES_PER_ROW: u32 = 10;

impl Scene for WgpuTutorial {
    async fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        queue: &wgpu::Queue,
    ) -> Box<Self> {
        let mut resource_manager = ResourceManager::new(device, config);

        // Shader setup
        resource_manager.load_shader(
            "shader.wgsl",
            "color_shader",
            device
        ).await;

        resource_manager.load_shader(
            "light.wgsl",
            "light_shader",
            device
        ).await;

        let texture_bind_group_layout = Texture::bind_group_layout(device);

        let obj_model = load_model("cube.obj", device, queue, &texture_bind_group_layout)
            .await
            .unwrap();

        // Camera
        let camera_controller = CameraController::new(4.0, 0.4, device, config);
        let camera_bind_group_layout = Camera::bind_group_layout(device);
        let camera_bind_group = camera_controller
            .camera
            .create_bind_group(&camera_bind_group_layout, device);

        // Instances - iterate through the amount we have, then create a buffer.
        let instances = (0..NUM_INSTANCES_PER_ROW)
            .flat_map(|z| {
                (0..NUM_INSTANCES_PER_ROW).map(move |x| {
                    let x = 3.0 * (x as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);
                    let z = 3.0 * (z as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);

                    let position = glam::Vec3 { x, y: 0.0, z };

                    let rotation = if position.is_nan() {
                        // Needed so an object at (0, 0, 0) doesn't get scaled to 0
                        // since Quaternions can effect scale if they're not "correct"
                        glam::Quat::from_axis_angle(glam::Vec3::Z, 0.0)
                    } else {
                        glam::Quat::from_axis_angle(position.normalize(), 45.0)
                    };

                    Instance { position, rotation }
                })
            })
            .collect::<Vec<_>>();
        let instance_buffer = InstanceRaw::create_buffer(&instances, device);

        let light = Light::new(
            glam::Vec3::new(2.0, 2.0, 2.0),
            glam::Vec3::new(1.0, 1.0, 1.0),
            device,
        );
        let light_bind_group_layout = Light::bind_group_layout(device);
        let light_bind_group = light.create_bind_group(&light_bind_group_layout, device);

        let render_pipeline = {
            let render_pipeline_layout =
                device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("WgpuTutorial Pipeline Layout"),
                    bind_group_layouts: &[
                        &texture_bind_group_layout,
                        &camera_bind_group_layout,
                        &light_bind_group_layout,
                    ],
                    push_constant_ranges: &[],
                });

            create_render_pipeline(
                device,
                &render_pipeline_layout,
                config.format,
                Some(Texture::DEPTH_FORMAT),
                &[ModelVertex::desc(), InstanceRaw::desc()],
                resource_manager.shaders.lock().unwrap().get("color_shader").unwrap(),
                Some("Color Render Pipeline")
            )
        };

        let light_render_pipeline = {
            let light_pipeline_layout =
                device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Light Pipeline Layout"),
                    bind_group_layouts: &[
                        &camera_bind_group_layout,
                        &light_bind_group_layout,
                    ],
                    push_constant_ranges: &[],
                });

            create_render_pipeline(
                device,
                &light_pipeline_layout,
                config.format,
                Some(Texture::DEPTH_FORMAT),
                &[ModelVertex::desc()],
                resource_manager.shaders.lock().unwrap().get("light_shader").unwrap(),
                Some("Light Render Pipeline"),
            )
        };

        Box::from(Self {
            resource_manager,
            render_pipeline,
            light_render_pipeline,
            camera_controller,
            instances,
            instance_buffer,
            obj_model,
            camera_bind_group,
            light_bind_group,
            light,
            mouse_pressed: false
        })
    }

    fn update(&mut self, queue: &wgpu::Queue, dt: Duration) {
        // Updating camera position
        self.camera_controller.update_camera(dt);
        self.camera_controller
            .camera_uniform
            .update_view_proj(&self.camera_controller.camera, &self.camera_controller.projection);
        queue.write_buffer(
            &self.camera_controller.camera.buffer,
            0,
            bytemuck::cast_slice(&[self.camera_controller.camera_uniform]),
        );

        // Update light position
        self.light.light_uniform.position =
            glam::Quat::from_axis_angle(glam::Vec3::new(0.0, 1.0, 0.0), 1.0 * dt.as_secs_f32())
                * self.light.light_uniform.position;

        queue.write_buffer(
            &self.light.buffer,
            0,
            bytemuck::cast_slice(&[self.light.light_uniform]),
        );
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
                }),
            ],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.resource_manager.depth_texture.view,
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

        render_pass.set_pipeline(&self.light_render_pipeline);
        render_pass.draw_light_model(
            &self.obj_model,
            &self.camera_bind_group,
            &self.light_bind_group,
        );

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.draw_model_instanced(
            &self.obj_model,
            0..self.instances.len() as u32,
            &self.camera_bind_group,
            &self.light_bind_group,
        );
    }

    fn input(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                self.camera_controller.keyboard_input(event);
            }
            WindowEvent::MouseWheel { delta, ..} => {
                self.camera_controller.scroll_input(delta);
            }
            WindowEvent::MouseInput {
                button: MouseButton::Left, state,
                ..
            } => {
                self.mouse_pressed = *state == ElementState::Pressed;
            }
            _ => (),
        }
    }

    fn device_input(&mut self, event: &DeviceEvent) {
        if let DeviceEvent::MouseMotion { delta} = event {
            if self.mouse_pressed {
                self.camera_controller.mouse_input(delta.0, delta.1);
            }
        }
    }

    fn resize(
        &mut self,
        new_size: PhysicalSize<u32>,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
    ) {
        self.resource_manager.depth_texture = Texture::create_depth_texture(device, config);
        self.camera_controller.projection.resize(new_size.width, new_size.height);
    }
}
