use std::sync::Arc;
use winit::event::{ Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use crate::scene::scene::Scene;
use crate::scene::wgpu_tutorial::WgpuTutorial;

/// The engine itself. Handles everything relating to the window and
/// ensuring that the right states are doing the things.
pub struct Aravoxel<'window> {
    surface: wgpu::Surface<'window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Arc<Window>,

    scene: WgpuTutorial,
}

impl Aravoxel<'_> {
    async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();

        // First thing's first: an instance, so we can create our surface (place to draw to) and adapter (GPU)
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        ).await.unwrap();

        // Now to set up the surface itself.
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .copied().find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            desired_maximum_frame_latency: 2,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let scene = WgpuTutorial::new(&device, &config, &queue).await;
        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            scene: *scene,
        }
    }

    fn window(&self) -> &Window {
        &self.window
    }

    /// As the name gives away - resizes the window.
    /// Reconfigures the surface.
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);

            // There might be surfaces in the scene that must be updated as well
            self.scene.resize(new_size, &self.device, &self.config);
        }
    }

    //TODO: Refactor this and make use of whatever scene's own input function
    fn input(&mut self, event: &WindowEvent) {
        self.scene.input(event);
    }

    fn update(&mut self) {
        self.scene.update(&self.queue);
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // We need to get the current SurfaceTexture to know where to draw to.
        let output = self.surface.get_current_texture()?;

        // We'll need a TextureView, too.
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        // The CommandEncoder builds a command buffer that we can send to the GPU later.
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render encoder"),
        });

        {
            self.scene.render(&view, &mut encoder);
        }

        // Send our buffer over to the GPU's rendering queue.
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

/// The only public function. Initializes the window and starts the loop.
pub async fn run() {
    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(WindowBuilder::new()
        .with_resizable(false)
        .with_title("aravoxel")
        .build(&event_loop)
        .unwrap());

    let mut aravoxel = Aravoxel::new(window).await;

    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run(move |event, elwt| {
        // Everything related to just window specific events.
        match event {
            // We wanna redraw every frame, so let's request redraws whenever the window wants to idle.
            Event::AboutToWait => {
                aravoxel.window().request_redraw();
            }
            Event::WindowEvent { event, window_id} => {
                if window_id == aravoxel.window().id() {
                    // This function handles scene specific input.
                    aravoxel.input(&event);
                    match event {
                        WindowEvent::RedrawRequested => {
                            aravoxel.update();
                            match aravoxel.render() {
                                Ok(_) => {}
                                // Reconfigure if we lose the surface.
                                Err(wgpu::SurfaceError::Lost) => aravoxel.resize(aravoxel.size),
                                // Out of memory, let's bail.
                                Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                                // Uhh... something's wrong.
                                Err(e) => eprintln!("{:?}", e),
                            }
                        }
                        WindowEvent::Resized(physical_size) => {
                            aravoxel.resize(physical_size);
                        }
                        WindowEvent::CloseRequested => elwt.exit(),
                        _ => ()
                    }
                }
            }
            _ => ()
        }
    }).expect("Window did, uh, something... bad?");
}