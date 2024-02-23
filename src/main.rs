
mod engine;
mod scene;
mod mesh;
mod entity;

fn main() {
    pollster::block_on(engine::aravoxel::run());
}
