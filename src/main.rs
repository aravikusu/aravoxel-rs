
mod engine;
mod scenes;

fn main() {
    pollster::block_on(engine::aravoxel::run());
}
