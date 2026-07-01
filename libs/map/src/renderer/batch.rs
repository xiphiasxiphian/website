use std::collections::HashMap;

use itertools::Itertools;
use log::warn;
use wgpu::{Buffer, BufferDescriptor, BufferUsages, Device, Queue};

use crate::renderer::{Renderable, bufferpool::BufferPool, texture::Texture, vertex::Vertex};

pub struct TextureBatch
{
    pub pool: BufferPool,
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl TextureBatch
{
    const MAX_VERTICES: usize = 4 * 1024;
    const MAX_INDICES: usize = 6 * 1024;

    pub fn new(device: &Device) -> Self
    {
        Self {
            pool: BufferPool::new(device, Self::MAX_VERTICES, Self::MAX_INDICES),
            vertices: Vec::with_capacity(Self::MAX_VERTICES),
            indices: Vec::with_capacity(Self::MAX_INDICES),
        }
    }

    pub fn push(&mut self, vertices: &[Vertex], indices: &[u32]) -> bool
    {
        if !self
            .pool
            .fits(self.vertices.len() + vertices.len(), self.indices.len() + indices.len())
        {
            warn!("TextureBatch full, renderable dropped");
            return false;
        }

        let offset = self.vertices.len() as u32;
        self.vertices.extend_from_slice(vertices);
        self.indices.extend(indices.iter().map(|i| i + offset));

        true
    }

    pub fn collect(renderables: &[Box<dyn Renderable>], dims: (f32, f32)) -> impl Iterator<Item = (&Texture, Self)>
    {
        // TODO: have some better id here than the texture ptr.
        let mut groups: HashMap<*const Texture, (&Texture, Self)> = HashMap::new();

        for renderable in renderables
        {
            let mesh = renderable.mesh(dims);
            let texture = renderable.texture();

            let indices = mesh.indices.iter().map(|&i| i as u32).collect_vec();

            groups
                .entry(texture as *const Texture)
                .or_insert_with(|| {
                    (
                        texture,
                        BatchGroup {
                            vertices: vec![],
                            indices: vec![],
                        },
                    )
                })
                .1
                .push(&mesh.vertices, &indices);
        }

        groups.into_values()
    }
}
