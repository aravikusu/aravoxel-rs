use crate::engine::resource::texture::Texture;
use crate::engine::util::create_shader_module;

///
pub struct Assets {
    pub depth_texture: Texture,
    pub color_shader: wgpu::ShaderModule,
}

impl Assets {
    pub async fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        Self {
            depth_texture: Texture::create_depth_texture(device, config),
            color_shader: create_shader_module(&device, "shader.wgsl", "Color shader").await
        }
    }
}