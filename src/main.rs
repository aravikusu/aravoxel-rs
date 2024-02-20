
mod engine;
mod scenes;
mod meshes;

fn main() {
    pollster::block_on(engine::aravoxel::run());
}
