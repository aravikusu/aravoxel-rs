use winit::event::KeyEvent;
use winit::keyboard::{KeyCode, PhysicalKey};

pub struct CameraController {
    pub(crate) camera: Camera,

    speed: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}

impl CameraController {
    pub(crate) fn new(
        speed: f32,
        eye: glam::Vec3,
        target: glam::Vec3,
        up: glam::Vec3,
        aspect: f32,
        fov_y: f32,
        z_near: f32,
        z_far: f32,
    ) -> Self {
        Self {
            camera: Camera::new(
                eye,
                target,
                up,
                aspect,
                fov_y,
                z_near,
                z_far,
            ),
            speed,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }

    pub(crate) fn keyboard_input(&mut self, event: &KeyEvent) {
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
    eye: glam::Vec3,
    target: glam::Vec3,
    up: glam::Vec3,
    aspect: f32,
    fov_y: f32,
    z_near: f32,
    z_far: f32,
}

impl Camera {
    pub fn new(
        eye: glam::Vec3,
        target: glam::Vec3,
        up: glam::Vec3,
        aspect: f32,
        fov_y: f32,
        z_near: f32,
        z_far: f32,
    ) -> Self {
        Self {
            eye,
            target,
            up,
            aspect,
            fov_y,
            z_near,
            z_far,
        }
    }
    pub fn build_view_projection_matrix(&self) -> glam::Mat4 {
        // View matrix - ensures the world is at the position and rotation of our camera.
        let view = glam::Mat4::look_at_rh(self.eye, self.target, self.up);
        // Projection matrix - warps everything to give us perspective
        let proj = glam::Mat4::perspective_rh(self.fov_y, self.aspect, self.z_near, self.z_far);

        proj * view
    }
}

/// To correctly store the view projection matrix in a shader.
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: glam::Mat4,
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: glam::Mat4::IDENTITY
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix();
    }
}