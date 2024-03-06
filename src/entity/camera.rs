use wgpu::util::DeviceExt;
use winit::event::KeyEvent;
use winit::keyboard::{KeyCode, PhysicalKey};

pub struct CameraController {
    pub camera: Camera,
    pub camera_uniform: CameraUniform,

    speed: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}

impl CameraController {
    pub fn new(
        speed: f32,
        eye: glam::Vec3,
        target: glam::Vec3,
        up: glam::Vec3,
        aspect: f32,
        fov_y: f32,
        z_near: f32,
        z_far: f32,
        device: &wgpu::Device,
    ) -> Self {
        let mut camera_uniform = CameraUniform::new();

        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let camera = Camera {
            eye,
            target,
            up,
            aspect,
            fov_y,
            z_near,
            z_far,
            buffer,
        };

        camera_uniform.update_view_proj(&camera);

        Self {
            camera,
            camera_uniform,
            speed,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }

    pub fn keyboard_input(&mut self, event: &KeyEvent) {
        match event.physical_key {
            PhysicalKey::Code(KeyCode::KeyW) | PhysicalKey::Code(KeyCode::ArrowUp) => {
                self.is_forward_pressed = event.state.is_pressed()
            }
            PhysicalKey::Code(KeyCode::KeyA) | PhysicalKey::Code(KeyCode::ArrowLeft) => {
                self.is_left_pressed = event.state.is_pressed()
            }
            PhysicalKey::Code(KeyCode::KeyS) | PhysicalKey::Code(KeyCode::ArrowDown) => {
                self.is_backward_pressed = event.state.is_pressed()
            }
            PhysicalKey::Code(KeyCode::KeyD) | PhysicalKey::Code(KeyCode::ArrowRight) => {
                self.is_right_pressed = event.state.is_pressed()
            }
            _ => ()
        }
    }

    pub fn update_camera(&mut self) {
        let forward = self.camera.target - self.camera.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.length();

        // Prevent glitching when the camera gets too close to the
        // center of the scene
        if self.is_forward_pressed && forward_mag > self.speed {
            self.camera.eye += forward_norm * self.speed;
        }
        if self.is_backward_pressed {
            self.camera.eye -= forward_norm * self.speed;
        }

        let right = forward_norm.cross(self.camera.up);

        // Recalc
        let forward = self.camera.target - self.camera.eye;
        let forward_mag = forward.length();

        if self.is_right_pressed {
            // Rescale the distance between the target and the eye so
            // that it doesn't change. The eye, therefore, still lies
            // on the circle made by the target and eye.
            self.camera.eye = self.camera.target - (forward + right * self.speed).normalize() * forward_mag;
        }
        if self.is_left_pressed {
            self.camera.eye = self.camera.target - (forward - right * self.speed).normalize() * forward_mag;
        }
    }
}

pub struct Camera {
    pub buffer: wgpu::Buffer,

    eye: glam::Vec3,
    target: glam::Vec3,
    up: glam::Vec3,
    aspect: f32,
    fov_y: f32,
    z_near: f32,
    z_far: f32,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> glam::Mat4 {
        // View matrix - ensures the world is at the position and rotation of our camera.
        let view = glam::Mat4::look_at_rh(self.eye, self.target, self.up);
        // Projection matrix - warps everything to give us perspective
        let proj = glam::Mat4::perspective_rh(self.fov_y, self.aspect, self.z_near, self.z_far);

        proj * view
    }

    pub fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("Camera bind group layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            }
        )
    }

    pub fn create_bind_group(&self, layout: &wgpu::BindGroupLayout, device: &wgpu::Device) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera bind group"),
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.buffer.as_entire_binding(),
                }
            ],
        })
    }
}

/// To correctly store the view projection matrix in a shader.
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_pos: glam::Vec4,
    view_proj: glam::Mat4,
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_pos: glam::Vec4::ZERO,
            view_proj: glam::Mat4::IDENTITY,
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_pos = camera.eye.extend(0.0);
        self.view_proj = camera.build_view_projection_matrix();
    }
}