use std::collections::HashMap;
use libnoise::prelude::*;
use crate::engine::resource::model::{Material, Mesh, Model, ModelVertex};
use crate::engine::resource::texture::Texture;
use crate::engine::util::load_texture;
use crate::voxel::util::{CHUNK_AREA, CHUNK_SIZE, CHUNK_SIZE_F32, CHUNK_VOL, create_chunk_indices, create_chunk_vertices};

#[derive(Debug, Clone)]
struct Voxel {
    pub is_solid: bool,
}

/// The ChunkModel holds both the Model that is
/// used to render our World, but also the Chunks themselves.
/// This is so we can easily access adjacent chunks during rendering,
/// as well as modify them based on player input.
pub struct ChunkModel {
    pub model: Vec<Model>,
    chunks: HashMap<glam::IVec3, Chunk>,
}

impl ChunkModel {
    pub fn new() -> Self {
        Self {
            model: Vec::new(),
            chunks: HashMap::new(),
        }
    }
    /// Build the Model.
    /// Iterates through all of the existing chunks to generate all of the
    /// vertices, indices, materials... etc.
    pub async fn build(
        &mut self,
        layout: &wgpu::BindGroupLayout,
        device: &wgpu::Device,
        queue: &wgpu::Queue
    ) {
        // We first iterate through our chunks and build all of the meshes
        // based on the chunk data we have
        let mut meshes: Vec<Mesh> = Vec::new();

        for (chunk_pos, chunk) in &self.chunks {
            let mut vertices: Vec<ModelVertex> = Vec::new();
            let mut indices: Vec<u32> = Vec::new();

            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    for z in 0..CHUNK_SIZE {
                        let index = (x + CHUNK_SIZE * z + CHUNK_AREA * y) as usize;

                        if let Some(voxel) = chunk.voxels.get(index) {
                            if !voxel.is_solid {
                                continue;
                            }

                            let pos = glam::Vec3::new(
                                x as f32 + chunk_pos.x as f32 * CHUNK_SIZE_F32,
                                y as f32 + chunk_pos.y as f32 * CHUNK_SIZE_F32,
                                z as f32 + chunk_pos.z as f32 * CHUNK_SIZE_F32,
                            );

                            vertices.extend(create_chunk_vertices(pos));
                            indices.extend(create_chunk_indices(vertices.len() as u32));
                        }
                    }
                }
            }

            meshes.push(Mesh {
                name: "".to_string(),
                vertex_buffer: ModelVertex::create_vertex_buffer("chunk", &vertices, device),
                index_buffer: ModelVertex::create_index_buffer("chunk", &indices, device),
                num_elements: indices.len() as u32,
                material: 0,
            });
        }

        // Right now we won't use any materials, but we need atleast one in the current implementation
        let diffuse_texture = load_texture("happy-tree.png", device, queue).await.unwrap();
        let bind_group = Texture::create_bind_group(&diffuse_texture, layout, device);
        let material = Material {
            name: "mat".to_string(),
            diffuse_texture,
            bind_group,
        };

        self.model.clear();
        self.model.push(Model {
            meshes,
            materials: vec![material],
        })
    }

    pub fn add_chunk(&mut self, chunk_pos: glam::IVec3) {
        let mut chunk = Chunk::new();
        chunk.generate(chunk_pos);

        self.chunks.insert(chunk_pos, chunk);
    }
}

#[derive(Debug)]
pub struct Chunk {
    voxels: Vec<Voxel>,
}

impl Chunk {
    pub fn new() -> Self {
        Self { voxels: Vec::new() }
    }

    pub fn generate(&mut self, chunk_pos: glam::IVec3) {
        let mut voxels: Vec<Voxel> = vec![Voxel { is_solid: false}; CHUNK_VOL as usize];
        let noise = Source::simplex(42069);

        let new_pos = chunk_pos * CHUNK_SIZE;

        for x in 0..CHUNK_SIZE {
            let wx = x + new_pos.x;
            for z in 0..CHUNK_SIZE {
                let wz = z + new_pos.z;

                let test = glam::Vec2::new(wx as f32, wz as f32) * 0.01;
                let world_height = (noise.sample([test.x as f64, test.y as f64]) * 32.0 + 32.0) as i32;
                let local_height = i32::min(world_height - new_pos.y, CHUNK_SIZE);

                for y in 0..local_height {
                    let wy = y + new_pos.y;
                    let index = (x + CHUNK_SIZE * z + CHUNK_AREA * y) as usize;

                    //voxels[index] = wy + 1;
                    voxels[index].is_solid = true;
                }
            }
        }
        self.voxels = voxels;
    }
}