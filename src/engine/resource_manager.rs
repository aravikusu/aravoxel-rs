use std::collections::HashMap;
use log::error;

use crate::engine::resource::model::Model;
use crate::engine::resource::texture::Texture;

/// The ResourceManager stores all of a scene's loaded things.
/// Such as models, shaders, bindgroups.
pub struct ResourceManager {
    models: HashMap<String, Model>,
    bind_group_layouts: HashMap<String, wgpu::BindGroupLayout>,
    
    /// The depth_texture is stored here seperately
    /// since it's more or less always used.
    pub depth_texture: Texture,
}

impl ResourceManager {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        Self {
            models: HashMap::new(),
            bind_group_layouts: HashMap::new(),
            depth_texture: Texture::create_depth_texture(device, config)
        }
    }
    
    pub fn store_layout(&mut self, layout: wgpu::BindGroupLayout, name: String) {
        self.bind_group_layouts.insert(name, layout);
    }

    /// Get a texture by name.
    pub fn get_texture(&mut self, name: &str) -> Option<&Model> {
        let texture = self.models.get(name);
        if texture.is_none() { error!("ResourceManager: There is no texture with the name {}", name) };

        texture
    }
    
    pub fn recreate_depth_texture(&mut self, device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) {
        self.depth_texture = Texture::create_depth_texture(device, config);
    }
}