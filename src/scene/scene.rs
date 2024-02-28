use winit::event::WindowEvent;

/// All of our Scenes implement this.
pub trait Scene {
    async fn new(_device: &wgpu::Device, _config: &wgpu::SurfaceConfiguration, queue: &wgpu::Queue) -> Box<Self>;

    fn update(&mut self, queue: &wgpu::Queue);

    /// Called by aravoxel every frame.
    fn render(&mut self, _view: &wgpu::TextureView, _encoder: &mut wgpu::CommandEncoder);

    fn input(&mut self, _event: &WindowEvent);

    fn resize(&mut self, _new_size: winit::dpi::PhysicalSize<u32>, device: &wgpu::Device, config: &wgpu::SurfaceConfiguration);
}