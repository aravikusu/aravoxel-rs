use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::engine::resource::texture::Texture;
use crate::engine::util::{create_shader_module, load_texture};

// A place to store shaders, textures, what have you... Resources.
pub struct ResourceManager {
    pub textures: Arc<Mutex<HashMap<String, Texture>>>,
    pub shaders: Arc<Mutex<HashMap<String, wgpu::ShaderModule>>>,

    pub depth_texture: Texture,
}

impl ResourceManager {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        Self {
            textures: Arc::new(Mutex::new(HashMap::new())),
            shaders: Arc::new(Mutex::new(HashMap::new())),
            depth_texture: Texture::create_depth_texture(device, config),
        }
    }

    #[allow(dead_code)]
    pub async fn load_texture(
        &mut self,
        file_name: &str,
        label: &str,
        device: &wgpu::Device,
        queue: &wgpu::Queue
    ) {
            let texture = load_texture(file_name, device, queue).await.unwrap();
            self.textures.lock().unwrap().insert(label.to_string(), texture);
    }

    pub async fn load_shader(
        &mut self,
        file_name: &str,
        label: &str,
        device: &wgpu::Device
    ) {
        let shader = create_shader_module(device, file_name, label).await;
        self.shaders.lock().unwrap().insert(label.to_string(), shader);
    }
}