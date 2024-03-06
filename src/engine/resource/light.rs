use wgpu::util::DeviceExt;

pub struct Light {
    pub light_uniform: LightUniform,
    pub buffer: wgpu::Buffer,
}

impl Light {
    pub fn new(position: glam::Vec3, color: glam::Vec3, device: &wgpu::Device) -> Self {
        let light_uniform = LightUniform::new(position, color);

        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Light"),
                contents: bytemuck::cast_slice(&[light_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        Self {
            light_uniform,
            buffer,
        }
    }

    pub fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: None,
            })
    }

    pub fn create_bind_group(&self, layout: &wgpu::BindGroupLayout, device: &wgpu::Device) -> wgpu::BindGroup {
        device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.buffer.as_entire_binding(),
                }],
                label: None,
            }
        )
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniform {
    pub position: glam::Vec3,
    // Uniforms in WGSL require 16 bytes (4 floats) of spacing, so we need to pad it out
    _padding: u32,
    color: glam::Vec3,
    _padding2: u32,
}

impl LightUniform {
    fn new(position: glam::Vec3, color: glam::Vec3) -> Self {
        Self {
            position,
            _padding: 0,
            color,
            _padding2: 0,
        }
    }
}