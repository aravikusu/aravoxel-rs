
mod engine;
mod scene;
mod mesh;

fn main() {
    pollster::block_on(engine::aravoxel::run());
}
