use winit::event::Event;

pub trait Scene {
    fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Box<Self> {
        todo!()
    }

    fn update(&mut self) {
        todo!()
    }

    // Called by aravoxel every frame.
    fn render(&mut self, view: &wgpu::TextureView, mut encoder: &mut wgpu::CommandEncoder) {
        todo!()
    }

    fn input(&mut self, event: &Event<()>) {
        todo!()
    }
}