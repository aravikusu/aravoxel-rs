use crate::engine::resource::model::ModelVertex;

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