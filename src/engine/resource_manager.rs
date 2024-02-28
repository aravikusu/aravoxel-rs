use std::collections::HashMap;
use log::error;
use crate::engine::resource::texture::Texture;

/// The ResourceManager stores all of a scene loaded textures and shaders,
/// as well as giving you an easy way to access them.
pub struct ResourceManager {
    textures: HashMap<String, Texture>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    /// Load a texture and store it in the ResourceManager.
    /// Also returns a reference to said texture.
    pub fn load_texture(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        file: &str,
        name: String
    ) {
        //let texture = Texture::new(&device, &queue, file).unwrap();

        //self.textures.insert(name.clone(), texture);

        //self.get_texture(&name).unwrap()
    }

    /// Get a texture by name.
    pub fn get_texture(&mut self, name: &str) -> Option<&Texture> {
        let texture = self.textures.get(name);
        match texture {
            None => error!("ResourceManager: There is no texture with the name {}", name),
            _ => ()
        };

        texture
    }

    /// Empty the ResourceManager. Generally used if the scene changes.
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.textures.clear();
    }
}