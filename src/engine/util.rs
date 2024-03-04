use std::io::{BufReader, Cursor};

use crate::engine::resource::model::{Material, Mesh, Model, ModelVertex};
use crate::engine::resource::texture::Texture;

pub trait Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static>;
}

pub async fn load_string(file_name: &str) -> anyhow::Result<String> {
    let path = std::path::Path::new(env!("OUT_DIR")).join("assets").join(file_name);
    let txt = std::fs::read_to_string(path)?;

    Ok(txt)
}

pub async fn load_binary(file_name: &str) -> anyhow::Result<Vec<u8>> {
    let path = std::path::Path::new(env!("OUT_DIR"))
        .join("assets")
        .join(file_name);
    println!("{:?}", path);
    let data = std::fs::read(path)?;

    Ok(data)
}

pub async fn load_texture(
    file_name: &str,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) -> anyhow::Result<Texture> {
    let data = load_binary(file_name).await?;
    Texture::create(device, queue, &data, file_name)
}

/// For loading .obj files and assembling them into something we can render.
pub async fn load_model(
    file_name: &str,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    layout: &wgpu::BindGroupLayout,
) -> anyhow::Result<Model> {
    let obj_text = load_string(format!("models/{file_name}").as_str()).await?;
    let obj_cursor = Cursor::new(obj_text);

    let mut obj_reader = BufReader::new(obj_cursor);

    let (models, obj_materials) = tobj::load_obj_buf_async(
        &mut obj_reader,
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
        |p| async move {
            let mat_text = load_string(format!("models/{}", &p).as_str()).await.unwrap();
            tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(mat_text)))
        },
    ).await?;

    // Get all the materials found in the .obj and make textures out of them
    let mut materials = Vec::new();
    for m in obj_materials? {
        let diffuse_texture = load_texture(
            format!(
                "models/{}",
                &m.diffuse_texture.unwrap()).as_str(),
            device,
            queue,
        ).await?;
        let bind_group = Texture::create_bind_group(&diffuse_texture, layout, device);

        materials.push(Material {
            name: m.name,
            diffuse_texture,
            bind_group,
        })
    };

    // Build meshes out of the models found inside of the .obj
    let meshes = models.into_iter().map(|m| {
        let vertices = (0..m.mesh.positions.len() / 3)
            .map(|i| ModelVertex {
                position: [
                    m.mesh.positions[i * 3],
                    m.mesh.positions[i * 3 + 1],
                    m.mesh.positions[i * 3 + 2],
                ],
                tex_coords: [m.mesh.texcoords[i * 2], 1.0 - m.mesh.texcoords[i * 2 + 1]],
                normal: [
                    m.mesh.normals[i * 3],
                    m.mesh.normals[i * 3 + 1],
                    m.mesh.normals[i * 3 + 2],
                ],
            }).collect::<Vec<_>>();

        let vertex_buffer = ModelVertex::create_vertex_buffer(file_name, &vertices, device);
        let index_buffer = ModelVertex::create_index_buffer(file_name, &m.mesh.indices, device);

        Mesh {
            name: file_name.to_string(),
            vertex_buffer,
            index_buffer,
            num_elements: m.mesh.indices.len() as u32,
            material: m.mesh.material_id.unwrap_or(0),
        }
    }).collect::<Vec<_>>();

    Ok(Model { meshes, materials })
}

pub async fn create_shader_module(device: &wgpu::Device, shader_file: &str, label: &str) -> wgpu::ShaderModule {
    let file = load_string(format!("shaders/{shader_file}").as_str()).await.unwrap();
    
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(label),
        source: wgpu::ShaderSource::Wgsl(file.into())
    })
}