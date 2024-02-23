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