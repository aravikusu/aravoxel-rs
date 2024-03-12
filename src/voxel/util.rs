use std::collections::HashMap;
use crate::engine::resource::model::ModelVertex;
use crate::voxel::chunk::Chunk;

pub const CHUNK_SIZE: i32 = 32;
pub const CHUNK_SIZE_F32: f32 = 32.0;
pub const CHUNK_SIZE_F64: f64 = 32.0;
pub const MAX_HEIGHT: f64 = CHUNK_SIZE_F64 / 2.0;

pub const CHUNK_AREA: i32 = CHUNK_SIZE * CHUNK_SIZE;
pub const CHUNK_VOL: i32 = CHUNK_AREA * CHUNK_SIZE;

/// Creates the ModelVertex vector as well as the index vector for our current Voxel.
/// 
/// * `chunk` - The Chunk this Voxel resides within.
/// * `local_pos` - This Voxel's local position within this Chunk.
/// * `world_pos` - This Voxel's *world position*. Necessary to correctly draw the vertices.
/// * `start_index` - The current amount of Vertices. Used to set the indices correctly.
/// * `world_chunks` - All of the chunks inside our world. Used so we can access another Chunk's
/// Voxels while we draw in case the neighboring Voxel isn't local to our current Chunk.
pub fn create_chunk_mesh_data(
    chunk: &Chunk,
    local_pos: glam::Vec3,
    world_pos: glam::Vec3,
    start_index: u32,
    world_chunks: &HashMap<glam::IVec3, Chunk>
) -> (Vec<ModelVertex>, Vec<u32>) {
    // Local position of Voxel within this Chunk
    let lx = local_pos.x as i32;
    let ly = local_pos.y as i32;
    let lz = local_pos.z as i32;
    
    // World position of Voxel
    let wx = world_pos.x;
    let wy = world_pos.y;
    let wz = world_pos.z;
    
    let mut model_verts: Vec<ModelVertex> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    
    let mut unique_verts: u32 = start_index;

    // Check if there's a visible voxel above above
    if chunk.is_void(glam::IVec3::new(lx, ly + 1, lz), world_chunks) {
        model_verts.extend(
            vec!(
                ModelVertex { position: [wx + -0.5, wy + 0.5, wz + -0.5], tex_coords: [0.0, 0.0], normal:[0.0, 1.0, 0.0]},
                ModelVertex { position: [wx + 0.5, wy + 0.5, wz + -0.5], tex_coords: [0.0, 0.0], normal:[0.0, 1.0, 0.0]},
                ModelVertex { position: [wx + 0.5, wy + 0.5, wz + 0.5], tex_coords: [0.0, 0.0], normal:[0.0, 1.0, 0.0]},
                ModelVertex { position: [wx + -0.5, wy + 0.5, wz + 0.5], tex_coords: [0.0, 0.0], normal:[0.0, 1.0, 0.0]},
            )
        );
        indices.extend(voxel_indices_extender(vec![0, 3, 1, 1, 3, 2], unique_verts));
        unique_verts += 4;
    }

    // Check under...
    if chunk.is_void(glam::IVec3::new(lx, ly - 1, lz), world_chunks) {
        model_verts.extend(
            vec!(
                ModelVertex { position: [wx + -0.5, wy + -0.5, wz + -0.5], tex_coords: [0.0, 0.0], normal:[0.0, -1.0, 0.0]},
                ModelVertex { position: [wx + 0.5, wy + -0.5, wz + -0.5], tex_coords: [0.0, 0.0], normal:[0.0, -1.0, 0.0]},
                ModelVertex { position: [wx + 0.5, wy + -0.5, wz + 0.5], tex_coords: [0.0, 0.0], normal:[0.0, -1.0, 0.0]},
                ModelVertex { position: [wx + -0.5, wy + -0.5, wz + 0.5], tex_coords: [0.0, 0.0], normal:[0.0, -1.0, 0.0]},
            )
        );

        indices.extend(voxel_indices_extender(vec![0, 1, 3, 1, 2, 3], unique_verts));
        unique_verts += 4;
    }

    // Right
    if chunk.is_void(glam::IVec3::new(lx + 1, ly, lz), world_chunks) {
        model_verts.extend(
            vec!(
                ModelVertex { position: [wx + 0.5, wy + -0.5, wz + -0.5], tex_coords: [0.0, 0.0], normal:[1.0, 0.0, 0.0]},
                ModelVertex { position: [wx + 0.5, wy + -0.5, wz + 0.5], tex_coords: [0.0, 0.0], normal:[1.0, 0.0, 0.0]},
                ModelVertex { position: [wx + 0.5, wy + 0.5, wz + 0.5], tex_coords: [0.0, 0.0], normal:[1.0, 0.0, 0.0]},
                ModelVertex { position: [wx + 0.5, wy + 0.5, wz + -0.5], tex_coords: [0.0, 0.0], normal:[1.0, 0.0, 0.0]},
            )
        );

        indices.extend(voxel_indices_extender(vec![0, 3, 1, 1, 3, 2], unique_verts));
        unique_verts += 4;
    }

    // Left
    if chunk.is_void(glam::IVec3::new(lx - 1, ly, lz), world_chunks) {
        model_verts.extend(
            vec!(
                ModelVertex { position: [wx + -0.5, wy + -0.5, wz + -0.5], tex_coords: [0.0, 0.0], normal:[-1.0, 0.0, 0.0]},
                ModelVertex { position: [wx + -0.5, wy + -0.5, wz + 0.5], tex_coords: [0.0, 0.0], normal:[-1.0, 0.0, 0.0]},
                ModelVertex { position: [wx + -0.5, wy + 0.5, wz + 0.5], tex_coords: [0.0, 0.0], normal:[-1.0, 0.0, 0.0]},
                ModelVertex { position: [wx + -0.5, wy + 0.5, wz + -0.5], tex_coords: [0.0, 0.0], normal:[-1.0, 0.0, 0.0]},
            )
        );

        indices.extend(voxel_indices_extender(vec![0, 1, 3, 1, 2, 3], unique_verts));
        unique_verts += 4;
    }

    // Behind
    if chunk.is_void(glam::IVec3::new(lx, ly, lz + 1), world_chunks) {
        model_verts.extend(
            vec!(
                ModelVertex { position: [wx + -0.5, wy + -0.5, wz + 0.5], tex_coords: [0.0, 0.0], normal:[0.0, 0.0, 1.0]},
                ModelVertex { position: [wx + -0.5, wy + 0.5, wz + 0.5], tex_coords: [0.0, 0.0], normal:[0.0, 0.0, 1.0]},
                ModelVertex { position: [wx + 0.5, wy + 0.5, wz + 0.5], tex_coords: [0.0, 0.0], normal:[0.0, 0.0, 1.0]},
                ModelVertex { position: [wx + 0.5, wy + -0.5, wz + 0.5], tex_coords: [0.0, 0.0], normal:[0.0, 0.0, 1.0]},
            )
        );

        indices.extend(voxel_indices_extender(vec![0, 3, 1, 1, 3, 2], unique_verts));
        unique_verts += 4;
    }

    // In front
    if chunk.is_void(glam::IVec3::new(lx, ly, lz - 1), world_chunks) {
        model_verts.extend(
            vec!(
                ModelVertex { position: [wx + -0.5, wy + -0.5, wz + -0.5], tex_coords: [0.0, 0.0], normal:[0.0, 0.0, -1.0]},
                ModelVertex { position: [wx + -0.5, wy + 0.5, wz + -0.5], tex_coords: [0.0, 0.0], normal:[0.0, 0.0, -1.0]},
                ModelVertex { position: [wx + 0.5, wy + 0.5, wz + -0.5], tex_coords: [0.0, 0.0], normal:[0.0, 0.0, -1.0]},
                ModelVertex { position: [wx + 0.5, wy + -0.5, wz + -0.5], tex_coords: [0.0, 0.0], normal:[0.0, 0.0, -1.0]},
            )
        );

        indices.extend(voxel_indices_extender(vec![0, 1, 3, 1, 2, 3], unique_verts));
        unique_verts += 4;
    }

    //println!("{:?}",indices);

    (model_verts, indices)
}

fn voxel_indices_extender(vec: Vec<u32>, extend_amount: u32) -> Vec<u32> {
    vec.into_iter()
        .map(|i| i + extend_amount)
        .collect()
}