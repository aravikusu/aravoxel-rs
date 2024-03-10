
mod engine;
mod scene;
mod entity;
mod voxel;

fn main() {
    pollster::block_on(engine::aravoxel::run());
}
