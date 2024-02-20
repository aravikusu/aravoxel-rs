use wgpu::util::DeviceExt;

use crate::engine::utils::Vertex;

pub struct Mesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: Option<wgpu::Buffer>,
    pub amount: u32,
}

impl Mesh {
    pub fn new(vertices: &[Vertex], indices: &[u16], device: &wgpu::Device) -> Self {
        let has_indices = !indices.is_empty();

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("WgpuTutorial Vertex Buffer"),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let index_buffer = if has_indices {
            Some(device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some("WgpuTutorial Index Buffer"),
                    contents: bytemuck::cast_slice(indices),
                    usage: wgpu::BufferUsages::INDEX,
                }
            ))
        } else {
            None
        };

        Self {
            vertex_buffer,
            index_buffer,
            amount: if has_indices { indices.len() } else { vertices.len() } as u32,
        }
    }
}