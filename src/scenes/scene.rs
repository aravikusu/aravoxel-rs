use winit::event::Event;

/// All of our Scenes implement this.
pub trait Scene {
    fn new(_device: &wgpu::Device, _config: &wgpu::SurfaceConfiguration) -> Box<Self>;

    fn update(&mut self);

    /// Called by aravoxel every frame.
    fn render(&mut self, _view: &wgpu::TextureView, _encoder: &mut wgpu::CommandEncoder);

    fn input(&mut self, _event: &Event<()>);
}