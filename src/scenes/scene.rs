use wgpu::{CommandEncoder, TextureView};
use winit::event::Event;

pub trait Scene {
    fn init() {
        todo!()
    }

    fn update(&mut self) {
        todo!()
    }

    // Called by aravoxel every frame.
    fn render(&mut self, view: &TextureView, mut encoder: &mut CommandEncoder) {
        todo!()
    }

    fn input(&mut self, event: &Event<()>) {
        todo!()
    }
}