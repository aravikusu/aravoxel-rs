use std::ops::Range;

use wgpu::util::DeviceExt;

use crate::engine::resource::texture::Texture;
use crate::engine::util::Vertex;

/// ModelVertex contains all Vertex information we want.
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelVertex {
    /// The position of this vertex.
    pub position: [f32; 3],
    /// The texture coordinates of this vertex.
    pub tex_coords: [f32; 2],
    /// The normal of this vertex.
    pub normal: [f32; 3],
}

impl ModelVertex {
    /// Create a Vertex Buffer out of a ModelVertex vector.
    pub fn create_vertex_buffer(
        file_name: &str,
        vertices: &[ModelVertex],
        device: &wgpu::Device,
    ) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{file_name} Vertex Buffer")),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }

    /// Create a Index Buffer out of a vector of indices.
    pub fn create_index_buffer(
        file_name: &str,
        indices: &[u32],
        device: &wgpu::Device,
    ) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{file_name} Index Buffer")),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        })
    }
}

impl Vertex for ModelVertex {
    /// Retrieve the VertexBufferLayout for the ModelVertex. Maps out the information for our shader.
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ModelVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

/// A Model. More or less what everything gets packed into.
/// Contains all the Meshes and said Mesh Materials.
pub struct Model {
    /// A list of all Meshes the Model contains.
    pub meshes: Vec<Mesh>,
    /// A list of all Materials the Model contains.
    pub materials: Vec<Material>,
}

/// A Material. A Texture + BindGroup essentially.
pub struct Material {
    /// Internal name of the Material.
    pub name: String,
    /// The actual Texture.
    pub diffuse_texture: Texture,
    /// The BindGroup for this Materials Texture.
    pub bind_group: wgpu::BindGroup,
}

/// A Mesh. A collection of vertices and indices.
pub struct Mesh {
    /// Internal name of the Mesh.
    pub name: String,
    /// A Buffer for all vertices for this Mesh.
    pub vertex_buffer: wgpu::Buffer,
    /// A Buffer for all indicies for this mesh.
    pub index_buffer: wgpu::Buffer,
    /// The amount of indices contained within.
    /// Used to determine how we should render things.
    pub num_elements: u32,
    /// The index of the Material.
    /// Since the vast majority of the time we'll pack a Mesh into a Model,
    /// we'll want to know which Material in the Model we want to use.
    ///
    /// This is specifically used to index that list. The `materials` of the Model.
    pub material: usize,
}

pub trait DrawModel<'a> {
    /// Takes a single Mesh and Material and draws it.
    /// It calls `draw_mesh_instanced` but specifying a single instance.
    fn draw_mesh(
        &mut self,
        mesh: &'a Mesh,
        material: &'a Material,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
    );

    /// Uses the supplied Mesh and Material and draws the amount of instances specified.
    fn draw_mesh_instanced(
        &mut self,
        mesh: &'a Mesh,
        material: &'a Material,
        instances: Range<u32>,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
    );

    // Takes a Model and draws it. It specifies a single instance.
    fn draw_model(
        &mut self,
        model: &'a Model,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
    );

    /// Uses a Model and draws the amount of specified instances.
    /// Uses `draw_mesh_instanced` in a loop.
    fn draw_model_instanced(
        &mut self,
        model: &'a Model,
        instances: Range<u32>,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
    );
}

impl<'a, 'b> DrawModel<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_mesh(
        &mut self,
        mesh: &'b Mesh,
        material: &'a Material,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
    ) {
        self.draw_mesh_instanced(mesh, material, 0..1, camera_bind_group, light_bind_group);
    }

    fn draw_mesh_instanced(
        &mut self,
        mesh: &'b Mesh,
        material: &'a Material,
        instances: Range<u32>,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
    ) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.set_bind_group(0, &material.bind_group, &[]);
        self.set_bind_group(1, camera_bind_group, &[]);
        self.set_bind_group(2, light_bind_group, &[]);
        self.draw_indexed(0..mesh.num_elements, 0, instances);
    }

    fn draw_model(
        &mut self,
        model: &'b Model,
        camera_bind_group: &'b wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
    ) {
        self.draw_model_instanced(model, 0..1, camera_bind_group, light_bind_group);
    }

    fn draw_model_instanced(
        &mut self,
        model: &'b Model,
        instances: Range<u32>,
        camera_bind_group: &'b wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
    ) {
        for mesh in &model.meshes {
            let material = &model.materials[mesh.material];
            self.draw_mesh_instanced(
                mesh,
                material,
                instances.clone(),
                camera_bind_group,
                light_bind_group,
            );
        }
    }
}

/// DrawLight is similar to DrawModel.
/// But since the our Light BindGroup doesn't
/// deal with any textures, they are omitted.
pub trait DrawLight<'a> {
    /// Takes a single Mesh and Material and draws it.
    /// It calls `draw_mesh_instanced` but specifying a single instance.
    fn draw_light_mesh(
        &mut self,
        mesh: &'a Mesh,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
    );

    /// Uses the supplied Mesh and Material and draws the amount of instances specified.
    fn draw_light_mesh_instanced(
        &mut self,
        mesh: &'a Mesh,
        instances: Range<u32>,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
    );

    // Takes a Model and draws it. It specifies a single instance.
    fn draw_light_model(
        &mut self,
        model: &'a Model,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
    );

    /// Uses a Model and draws the amount of specified instances.
    /// Uses `draw_mesh_instanced` in a loop.
    fn draw_light_model_instanced(
        &mut self,
        model: &'a Model,
        instances: Range<u32>,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
    );
}


impl<'a, 'b> DrawLight<'b> for wgpu::RenderPass<'a>
    where
        'b: 'a,
{
    fn draw_light_mesh(
        &mut self,
        mesh: &'b Mesh,
        camera_bind_group: &'b wgpu::BindGroup,
        light_bind_group: &'b wgpu::BindGroup,
    ) {
        self.draw_light_mesh_instanced(mesh, 0..1, camera_bind_group, light_bind_group);
    }

    fn draw_light_mesh_instanced(
        &mut self,
        mesh: &'b Mesh,
        instances: Range<u32>,
        camera_bind_group: &'b wgpu::BindGroup,
        light_bind_group: &'b wgpu::BindGroup,
    ) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.set_bind_group(0, camera_bind_group, &[]);
        self.set_bind_group(1, light_bind_group, &[]);
        self.draw_indexed(0..mesh.num_elements, 0, instances);
    }

    fn draw_light_model(
        &mut self,
        model: &'b Model,
        camera_bind_group: &'b wgpu::BindGroup,
        light_bind_group: &'b wgpu::BindGroup,
    ) {
        self.draw_light_model_instanced(model, 0..1, camera_bind_group, light_bind_group);
    }
    fn draw_light_model_instanced(
        &mut self,
        model: &'b Model,
        instances: Range<u32>,
        camera_bind_group: &'b wgpu::BindGroup,
        light_bind_group: &'b wgpu::BindGroup,
    ) {
        for mesh in &model.meshes {
            self.draw_light_mesh_instanced(mesh, instances.clone(), camera_bind_group, light_bind_group);
        }
    }
}