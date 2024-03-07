use std::f32::consts::FRAC_PI_2;
use std::time::Duration;
use wgpu::util::DeviceExt;
use winit::event::{ElementState, KeyEvent, MouseScrollDelta};
use winit::keyboard::{KeyCode, PhysicalKey};

const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.0001;

#[derive(Debug)]
pub struct CameraController {
    pub camera: Camera,
    pub projection: Projection,
    pub camera_uniform: CameraUniform,

    amount_left: f32,
    amount_right: f32,
    amount_forward: f32,
    amount_backward: f32,
    amount_up: f32,
    amount_down: f32,
    rotate_horizontal: f32,
    rotate_vertical: f32,
    scroll: f32,
    speed: f32,
    sensitivity: f32,
}

impl CameraController {
    pub fn new(
        speed: f32,
        sensitivity: f32,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration
    ) -> Self {
        let mut camera_uniform = CameraUniform::new();

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera = Camera::new(
            buffer,
            glam::Vec3::new(0.0, 0.5, 10.0),
            -90.0,
            -20.0
        );

        let projection = Projection::new(
            config.width,
            config.height,
            45.0,
            0.1,
            100.0
        );

        camera_uniform.update_view_proj(&camera, &projection);

        Self {
            camera,
            camera_uniform,
            projection,
            amount_left: 0.0,
            amount_right: 0.0,
            amount_forward: 0.0,
            amount_backward: 0.0,
            amount_up: 0.0,
            amount_down: 0.0,
            rotate_horizontal: 0.0,
            rotate_vertical: 0.0,
            scroll: 0.0,
            speed,
            sensitivity,
        }
    }

    pub fn keyboard_input(&mut self, event: &KeyEvent) {
        let amount = if event.state == ElementState::Pressed {
            1.0
        } else {
            0.0
        };

        match event.physical_key {
            PhysicalKey::Code(KeyCode::KeyW) | PhysicalKey::Code(KeyCode::ArrowUp) => {
                self.amount_forward = amount;
            }
            PhysicalKey::Code(KeyCode::KeyA) | PhysicalKey::Code(KeyCode::ArrowLeft) => {
                self.amount_left = amount;
            }
            PhysicalKey::Code(KeyCode::KeyS) | PhysicalKey::Code(KeyCode::ArrowDown) => {
                self.amount_backward = amount;
            }
            PhysicalKey::Code(KeyCode::KeyD) | PhysicalKey::Code(KeyCode::ArrowRight) => {
                self.amount_right = amount;
            }
            PhysicalKey::Code(KeyCode::Space) => {
                self.amount_up = amount;
            }
            PhysicalKey::Code(KeyCode::ShiftLeft) => {
                self.amount_down = amount;
            }
            _ => (),
        }
    }

    pub fn mouse_input(&mut self, mouse_dx: f64, mouse_dy: f64) {
        self.rotate_horizontal = mouse_dx as f32;
        self.rotate_vertical = mouse_dy as f32;
    }

    pub fn scroll_input(&mut self, delta: &MouseScrollDelta) {
        self.scroll = -match delta {
            MouseScrollDelta::LineDelta(_, scroll) => scroll * 100.0,
            MouseScrollDelta::PixelDelta(winit::dpi::PhysicalPosition { y: scroll, .. }) => {
                *scroll as f32
            }
        }
    }

    pub fn update_camera(&mut self, dt: Duration) {
        let dt = dt.as_secs_f32();
        let (yaw_sin, yaw_cos) = self.camera.yaw.sin_cos();

        // Moving forward/backward, left/right
        let forward = glam::Vec3::new(yaw_cos, 0.0, yaw_sin).normalize();
        let right = glam::Vec3::new(-yaw_sin, 0.0, yaw_cos).normalize();
        self.camera.position += forward * (self.amount_forward - self.amount_backward) * self.speed * dt;
        self.camera.position += right * (self.amount_right - self.amount_left) * self.speed * dt;

        // "zoom"
        let (pitch_sin, pitch_cos) = self.camera.pitch.sin_cos();
        let scroll = glam::Vec3::new(pitch_cos * yaw_cos, pitch_sin, pitch_cos * yaw_sin).normalize();
        self.camera.position += scroll * self.scroll * self.sensitivity * dt;

        // Move up/down
        self.camera.position.y += (self.amount_up - self.amount_down) * self.speed * dt;

        // Rotate
        self.camera.yaw += self.rotate_horizontal * self.sensitivity * dt;
        self.camera.pitch += -self.rotate_vertical * self.sensitivity * dt;

        // Set to 0 just in case there has been frame lag
        self.rotate_vertical = 0.0;
        self.rotate_horizontal = 0.0;

        // Restrict from going too high
        if self.camera.pitch < -SAFE_FRAC_PI_2 {
            self.camera.pitch = -SAFE_FRAC_PI_2
        } else if self.camera.pitch > SAFE_FRAC_PI_2 {
            self.camera.pitch = SAFE_FRAC_PI_2
        }
    }
}

#[derive(Debug)]
pub struct Camera {
    pub buffer: wgpu::Buffer,

    pub position: glam::Vec3,
    yaw: f32,
    pitch: f32,
}

impl Camera {
    pub fn new(buffer: wgpu::Buffer, position: glam::Vec3, yaw: f32, pitch: f32) -> Self {
        Self {
            buffer,
            position,
            yaw,
            pitch,
        }
    }

    pub fn calc_matrix(&self) -> glam::Mat4 {
        let (sin_pitch, cos_pitch) = self.pitch.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.sin_cos();

        glam::Mat4::look_to_rh(
            self.position,
            glam::Vec3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize(),
            glam::Vec3::Y,
        )
    }

    pub fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Camera bind group layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        })
    }

    pub fn create_bind_group(
        &self,
        layout: &wgpu::BindGroupLayout,
        device: &wgpu::Device,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera bind group"),
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: self.buffer.as_entire_binding(),
            }],
        })
    }
}

/// Projection only changes on window resize,
/// so it might as well be it's own thing.
#[derive(Debug)]
pub struct Projection {
    aspect: f32,
    fov_y: f32,
    z_near: f32,
    z_far: f32,
}

impl Projection {
    pub fn new(width: u32, height: u32, fov_y: f32, z_near: f32, z_far: f32) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fov_y,
            z_near,
            z_far,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }

    pub fn calc_matrix(&self) -> glam::Mat4 {
        glam::Mat4::perspective_rh(self.fov_y, self.aspect, self.z_near, self.z_far)
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

    pub fn update_view_proj(&mut self, camera: &Camera, projection: &Projection) {
        self.view_pos = camera.position.extend(0.0);
        self.view_proj = projection.calc_matrix() * camera.calc_matrix();
    }
}
