use crate::engine::resource::model::ModelVertex;
use crate::voxel::chunk::Chunk;

pub const CHUNK_SIZE: i32 = 32;
pub const CHUNK_SIZE_F32: f32 = 32.0;
pub const CHUNK_SIZE_F64: f64 = 32.0;
pub const MAX_HEIGHT: f64 = CHUNK_SIZE_F64 / 2.0;

pub const CHUNK_AREA: i32 = CHUNK_SIZE * CHUNK_SIZE;
pub const CHUNK_VOL: i32 = CHUNK_AREA * CHUNK_SIZE;

pub fn create_chunk_vertices(pos: glam::Vec3) -> Vec<ModelVertex> {
    let x = pos.x;
    let y = pos.y;
    let z = pos.z;

    vec![
        // Top
        ModelVertex { position: [x + -0.5, y + 0.5, z + -0.5], tex_coords: [0.0, 0.0], normal:[0.0, 1.0, 0.0]},
        ModelVertex { position: [x + 0.5, y + 0.5, z + -0.5], tex_coords: [0.0, 0.0], normal:[0.0, 1.0, 0.0]},
        ModelVertex { position: [x + 0.5, y + 0.5, z + 0.5], tex_coords: [0.0, 0.0], normal:[0.0, 1.0, 0.0]},
        ModelVertex { position: [x + -0.5, y + 0.5, z + 0.5], tex_coords: [0.0, 0.0], normal:[0.0, 1.0, 0.0]},
        // Bottom
        ModelVertex { position: [x + -0.5, y + -0.5, z + -0.5], tex_coords: [0.0, 0.0], normal:[0.0, -1.0, 0.0]},
        ModelVertex { position: [x + 0.5, y + -0.5, z + -0.5], tex_coords: [0.0, 0.0], normal:[0.0, -1.0, 0.0]},
        ModelVertex { position: [x + 0.5, y + -0.5, z + 0.5], tex_coords: [0.0, 0.0], normal:[0.0, -1.0, 0.0]},
        ModelVertex { position: [x + -0.5, y + -0.5, z + 0.5], tex_coords: [0.0, 0.0], normal:[0.0, -1.0, 0.0]},
        // Right
        ModelVertex { position: [x + 0.5, y + -0.5, z + -0.5], tex_coords: [0.0, 0.0], normal:[1.0, 0.0, 0.0]},
        ModelVertex { position: [x + 0.5, y + -0.5, z + 0.5], tex_coords: [0.0, 0.0], normal:[1.0, 0.0, 0.0]},
        ModelVertex { position: [x + 0.5, y + 0.5, z + 0.5], tex_coords: [0.0, 0.0], normal:[1.0, 0.0, 0.0]},
        ModelVertex { position: [x + 0.5, y + 0.5, z + -0.5], tex_coords: [0.0, 0.0], normal:[1.0, 0.0, 0.0]},
        // Left
        ModelVertex { position: [x + -0.5, y + -0.5, z + -0.5], tex_coords: [0.0, 0.0], normal:[-1.0, 0.0, 0.0]},
        ModelVertex { position: [x + -0.5, y + -0.5, z + 0.5], tex_coords: [0.0, 0.0], normal:[-1.0, 0.0, 0.0]},
        ModelVertex { position: [x + -0.5, y + 0.5, z + 0.5], tex_coords: [0.0, 0.0], normal:[-1.0, 0.0, 0.0]},
        ModelVertex { position: [x + -0.5, y + 0.5, z + -0.5], tex_coords: [0.0, 0.0], normal:[-1.0, 0.0, 0.0]},
        // Back
        ModelVertex { position: [x + -0.5, y + -0.5, z + 0.5], tex_coords: [0.0, 0.0], normal:[0.0, 0.0, 1.0]},
        ModelVertex { position: [x + -0.5, y + 0.5, z + 0.5], tex_coords: [0.0, 0.0], normal:[0.0, 0.0, 1.0]},
        ModelVertex { position: [x + 0.5, y + 0.5, z + 0.5], tex_coords: [0.0, 0.0], normal:[0.0, 0.0, 1.0]},
        ModelVertex { position: [x + 0.5, y + -0.5, z + 0.5], tex_coords: [0.0, 0.0], normal:[0.0, 0.0, 1.0]},
        // Forward
        ModelVertex { position: [x + -0.5, y + -0.5, z + -0.5], tex_coords: [0.0, 0.0], normal:[0.0, 0.0, -1.0]},
        ModelVertex { position: [x + -0.5, y + 0.5, z + -0.5], tex_coords: [0.0, 0.0], normal:[0.0, 0.0, -1.0]},
        ModelVertex { position: [x + 0.5, y + 0.5, z + -0.5], tex_coords: [0.0, 0.0], normal:[0.0, 0.0, -1.0]},
        ModelVertex { position: [x + 0.5, y + -0.5, z + -0.5], tex_coords: [0.0, 0.0], normal:[0.0, 0.0, -1.0]},
    ]
}

pub fn create_chunk_indices(start_index: u32) -> Vec<u32> {
    vec![
        0, 3, 1, 1, 3, 2, // triangles making up the top (+y) facing side.
        4, 5, 7, 5, 6, 7, // bottom (-y)
        8, 11, 9, 9, 11, 10, // right (+x)
        12, 13, 15, 13, 14, 15, // left (-x)
        16, 19, 17, 17, 19, 18, // back (+z)
        20, 21, 23, 21, 22, 23, // forward (-z)
    ]
        .into_iter()
        .map(|i| i + start_index)
        .collect()
}

/// Creates the ModelVertex array as well as the index array for our current voxel.
/// The entire Chunk is sent in so that we can compare it with its neighbors
/// In order to properly decide on if we should render a side of the voxel or not.
pub fn create_chunk_mesh_data(chunk: &Chunk, pos: glam::Vec3, start_index: u32) -> (Vec<ModelVertex>, Vec<u32>) {
    let x = pos.x as i32;
    let y = pos.y as i32;
    let z = pos.z as i32;
    let mut model_verts: Vec<ModelVertex> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    let mut unique_verts: u32 = start_index;

    // Check if there's a visible voxel above above
    if !chunk.voxel_visible(x + CHUNK_SIZE * z + CHUNK_AREA * (y + 1)) {
        model_verts.extend(
            vec!(
                ModelVertex { position: [pos.x + -0.5, pos.y + 0.5, pos.z + -0.5], tex_coords: [0.0, 0.0], normal:[0.0, 1.0, 0.0]},
                ModelVertex { position: [pos.x + 0.5, pos.y + 0.5, pos.z + -0.5], tex_coords: [0.0, 0.0], normal:[0.0, 1.0, 0.0]},
                ModelVertex { position: [pos.x + 0.5, pos.y + 0.5, pos.z + 0.5], tex_coords: [0.0, 0.0], normal:[0.0, 1.0, 0.0]},
                ModelVertex { position: [pos.x + -0.5, pos.y + 0.5, pos.z + 0.5], tex_coords: [0.0, 0.0], normal:[0.0, 1.0, 0.0]},
            )
        );
        indices.extend(voxel_indices_extender(vec![0, 3, 1, 1, 3, 2], unique_verts));
        unique_verts += 4;
    }

    // Check under...
    if !chunk.voxel_visible(x + CHUNK_SIZE * z + CHUNK_AREA * (y + -1)) {
        model_verts.extend(
            vec!(
                ModelVertex { position: [pos.x + -0.5, pos.y + -0.5, pos.z + -0.5], tex_coords: [0.0, 0.0], normal:[0.0, -1.0, 0.0]},
                ModelVertex { position: [pos.x + 0.5, pos.y + -0.5, pos.z + -0.5], tex_coords: [0.0, 0.0], normal:[0.0, -1.0, 0.0]},
                ModelVertex { position: [pos.x + 0.5, pos.y + -0.5, pos.z + 0.5], tex_coords: [0.0, 0.0], normal:[0.0, -1.0, 0.0]},
                ModelVertex { position: [pos.x + -0.5, pos.y + -0.5, pos.z + 0.5], tex_coords: [0.0, 0.0], normal:[0.0, -1.0, 0.0]},
            )
        );

        indices.extend(voxel_indices_extender(vec![0, 1, 3, 1, 2, 3], unique_verts));
        unique_verts += 4;
    }

    // Right
    if !chunk.voxel_visible((x + 1) + CHUNK_SIZE * z + CHUNK_AREA * y) {
        model_verts.extend(
            vec!(
                ModelVertex { position: [pos.x + 0.5, pos.y + -0.5, pos.z + -0.5], tex_coords: [0.0, 0.0], normal:[1.0, 0.0, 0.0]},
                ModelVertex { position: [pos.x + 0.5, pos.y + -0.5, pos.z + 0.5], tex_coords: [0.0, 0.0], normal:[1.0, 0.0, 0.0]},
                ModelVertex { position: [pos.x + 0.5, pos.y + 0.5, pos.z + 0.5], tex_coords: [0.0, 0.0], normal:[1.0, 0.0, 0.0]},
                ModelVertex { position: [pos.x + 0.5, pos.y + 0.5, pos.z + -0.5], tex_coords: [0.0, 0.0], normal:[1.0, 0.0, 0.0]},
            )
        );

        indices.extend(voxel_indices_extender(vec![0, 3, 1, 1, 3, 2], unique_verts));
        unique_verts += 4;
    }

    // Left
    if !chunk.voxel_visible((x - 1) + CHUNK_SIZE * z + CHUNK_AREA * y) {
        model_verts.extend(
            vec!(
                ModelVertex { position: [pos.x + -0.5, pos.y + -0.5, pos.z + -0.5], tex_coords: [0.0, 0.0], normal:[-1.0, 0.0, 0.0]},
                ModelVertex { position: [pos.x + -0.5, pos.y + -0.5, pos.z + 0.5], tex_coords: [0.0, 0.0], normal:[-1.0, 0.0, 0.0]},
                ModelVertex { position: [pos.x + -0.5, pos.y + 0.5, pos.z + 0.5], tex_coords: [0.0, 0.0], normal:[-1.0, 0.0, 0.0]},
                ModelVertex { position: [pos.x + -0.5, pos.y + 0.5, pos.z + -0.5], tex_coords: [0.0, 0.0], normal:[-1.0, 0.0, 0.0]},
            )
        );

        indices.extend(voxel_indices_extender(vec![0, 1, 3, 1, 2, 3], unique_verts));
        unique_verts += 4;
    }

    // Behind
    if !chunk.voxel_visible(x + CHUNK_SIZE * (z + 1) + CHUNK_AREA * y) {
        model_verts.extend(
            vec!(
                ModelVertex { position: [pos.x + -0.5, pos.y + -0.5, pos.z + 0.5], tex_coords: [0.0, 0.0], normal:[0.0, 0.0, 1.0]},
                ModelVertex { position: [pos.x + -0.5, pos.y + 0.5, pos.z + 0.5], tex_coords: [0.0, 0.0], normal:[0.0, 0.0, 1.0]},
                ModelVertex { position: [pos.x + 0.5, pos.y + 0.5, pos.z + 0.5], tex_coords: [0.0, 0.0], normal:[0.0, 0.0, 1.0]},
                ModelVertex { position: [pos.x + 0.5, pos.y + -0.5, pos.z + 0.5], tex_coords: [0.0, 0.0], normal:[0.0, 0.0, 1.0]},
            )
        );

        indices.extend(voxel_indices_extender(vec![0, 3, 1, 1, 3, 2], unique_verts));
        unique_verts += 4;
    }

    // In front
    if !chunk.voxel_visible(x + CHUNK_SIZE * (z - 1) + CHUNK_AREA * y) {
        model_verts.extend(
            vec!(
                ModelVertex { position: [pos.x + -0.5, pos.y + -0.5, pos.z + -0.5], tex_coords: [0.0, 0.0], normal:[0.0, 0.0, -1.0]},
                ModelVertex { position: [pos.x + -0.5, pos.y + 0.5, pos.z + -0.5], tex_coords: [0.0, 0.0], normal:[0.0, 0.0, -1.0]},
                ModelVertex { position: [pos.x + 0.5, pos.y + 0.5, pos.z + -0.5], tex_coords: [0.0, 0.0], normal:[0.0, 0.0, -1.0]},
                ModelVertex { position: [pos.x + 0.5, pos.y + -0.5, pos.z + -0.5], tex_coords: [0.0, 0.0], normal:[0.0, 0.0, -1.0]},
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