use crate::engine::resource::model::{Material, Mesh, Model, ModelVertex};
use crate::engine::resource::texture::Texture;
use crate::engine::util::load_texture;
use crate::voxel::util::{
    create_chunk_mesh_data, CHUNK_AREA, CHUNK_SIZE, CHUNK_SIZE_F32, CHUNK_VOL,
};
use libnoise::prelude::*;
use std::collections::HashMap;

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
        queue: &wgpu::Queue,
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

                            let local_pos = glam::Vec3::new(x as f32, y as f32, z as f32);

                            let world_pos = glam::Vec3::new(
                                x as f32 + chunk_pos.x as f32 * CHUNK_SIZE_F32,
                                y as f32 + chunk_pos.y as f32 * CHUNK_SIZE_F32,
                                z as f32 + chunk_pos.z as f32 * CHUNK_SIZE_F32,
                            );

                            //println!("{}", world_pos);

                            let (m_vert, m_idx) = create_chunk_mesh_data(
                                chunk,
                                local_pos,
                                world_pos,
                                vertices.len() as u32,
                                &self.chunks,
                            );

                            vertices.extend(m_vert);
                            indices.extend(m_idx);
                            //vertices.extend(create_chunk_vertices(pos));
                            //indices.extend(create_chunk_indices(vertices.len() as u32));
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
        let mut chunk = Chunk::new(chunk_pos);
        chunk.generate(chunk_pos);

        self.chunks.insert(chunk_pos, chunk);
    }
}

#[derive(Debug)]
pub struct Chunk {
    position: glam::IVec3,
    voxels: Vec<Voxel>,
}

impl Chunk {
    pub fn new(position: glam::IVec3) -> Self {
        Self {
            position,
            voxels: Vec::new()
        }
    }

    /// Generate a Chunk based on current position.
    pub fn generate(&mut self, chunk_pos: glam::IVec3) {
        let mut voxels: Vec<Voxel> = vec![Voxel { is_solid: false }; CHUNK_VOL as usize];
        let noise = Source::simplex(42069).fbm(1, 1.0, 2.0, 0.5);

        let new_pos = chunk_pos * CHUNK_SIZE;

        for x in 0..CHUNK_SIZE {
            let wx = x + new_pos.x;
            for z in 0..CHUNK_SIZE {
                let wz = z + new_pos.z;

                let test = glam::Vec2::new(wx as f32, wz as f32) * 0.01;
                let world_height =
                    (noise.sample([test.x as f64, test.y as f64]) * 32.0 + 32.0) as i32;
                let local_height = i32::min(world_height - new_pos.y, CHUNK_SIZE);

                for y in 0..local_height {
                    let _wy = y + new_pos.y;
                    let index = (x + CHUNK_SIZE * z + CHUNK_AREA * y) as usize;

                    //voxels[index] = wy + 1;
                    voxels[index].is_solid = true;
                }
            }
        }
        self.voxels = voxels;
    }

    pub fn is_void(
        &self,
        voxel_pos: glam::IVec3,
        world_chunks: &HashMap<glam::IVec3, Chunk>
    ) -> bool {
        let x = voxel_pos.x;
        let y = voxel_pos.y;
        let z = voxel_pos.z;

        // First check if the position is local
        if (0..CHUNK_SIZE).contains(&x)
            && (0..CHUNK_SIZE).contains(&y)
            && (0..CHUNK_SIZE).contains(&z)
        {
            // Voxel exists inside of our chunk. Get the index and check it.
            let idx = (x + CHUNK_SIZE * z + CHUNK_AREA * y) as usize;
            if let Some(voxel) = self.voxels.get(idx) {
                return !voxel.is_solid;
            }
        } else {
            // Voxel exceeds chunk boundaries.
            // We need to know check the neighboring Chunk's voxel
            // to find out if we should draw or not.
            // FIXME: Probably refactor, this isn't great...
            
            let c_pos = self.position;
            let mut neighbor_chunk_idx = glam::IVec3::ZERO;
            let mut neightbor_voxel_pos = glam::IVec3::ZERO;
            if x > (CHUNK_SIZE - 1) {
                neighbor_chunk_idx = glam::IVec3::new(c_pos.x + 1, c_pos.y, c_pos.z);
                neightbor_voxel_pos = glam::IVec3::new(0, y, z);
            } else if x < 0 {
                neighbor_chunk_idx = glam::IVec3::new(c_pos.x - 1, c_pos.y, c_pos.z);
                neightbor_voxel_pos = glam::IVec3::new(31, y, z);
            }

            if y > (CHUNK_SIZE - 1) {
                neighbor_chunk_idx = glam::IVec3::new(c_pos.x, c_pos.y + 1, c_pos.z);
                neightbor_voxel_pos = glam::IVec3::new(x, 0, z);
            } else if y < 0 {
                neighbor_chunk_idx = glam::IVec3::new(c_pos.x, c_pos.y - 1, c_pos.z);
                neightbor_voxel_pos = glam::IVec3::new(x, 31, z);
            }

            if z > (CHUNK_SIZE - 1) {
                neighbor_chunk_idx = glam::IVec3::new(c_pos.x, c_pos.y, c_pos.z + 1);
                neightbor_voxel_pos = glam::IVec3::new(x, y, 0);
            } else if z < 0 {
                neighbor_chunk_idx = glam::IVec3::new(c_pos.x, c_pos.y, c_pos.z - 1);
                neightbor_voxel_pos = glam::IVec3::new(x, y, 31);
            }

            return Chunk::check_neighboring_chunk(neighbor_chunk_idx, neightbor_voxel_pos, world_chunks)
        }
        true
    }
    
    /// Tries to check the desired Voxel inside of a specified Chunk.
    /// 
    /// * `chunk_idx`: The key for the Chunk we're interested in.
    /// * `voxel_pos`: The local position of the Voxel we're interested in.
    /// * `world_chunks`: All of the loaded chunks located in our world.
    fn check_neighboring_chunk(
        chunk_idx: glam::IVec3,
        voxel_pos: glam::IVec3,
        world_chunks: &HashMap<glam::IVec3, Chunk>
    ) -> bool {
        let x = voxel_pos.x;
        let y = voxel_pos.y;
        let z = voxel_pos.z;
        match world_chunks.get(&chunk_idx) {
            Some(chunk) => {
                let voxel_idx = (x + CHUNK_SIZE * z + CHUNK_AREA * y) as usize;

                if let Some(voxel) = chunk.voxels.get(voxel_idx) {
                    return !voxel.is_solid;
                }
                
                true
            }
            None => false
        }
    }
}
