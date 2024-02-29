
mod engine;
mod scene;
mod entity;

fn main() {
    pollster::block_on(engine::aravoxel::run());
}
