use winit::event::{Event, WindowEvent};

/// All of our Scenes implement this.
pub trait Scene {
    fn new(_device: &wgpu::Device, _config: &wgpu::SurfaceConfiguration, queue: &wgpu::Queue) -> Box<Self>;

    fn update(&mut self, queue: &wgpu::Queue);

    /// Called by aravoxel every frame.
    fn render(&mut self, _view: &wgpu::TextureView, _encoder: &mut wgpu::CommandEncoder);

    fn input(&mut self, _event: &WindowEvent);
}