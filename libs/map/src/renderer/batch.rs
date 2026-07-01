use std::collections::HashMap;

use itertools::Itertools;
use wgpu::{Buffer, BufferDescriptor, BufferUsages, Device, Queue};

use crate::renderer::{Renderable, texture::Texture, vertex::Vertex};

pub struct BatchGroup
{
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl BatchGroup
{
    fn push(&mut self, vertices: &[Vertex], indices: &[u32])
    {
        let offset = self.vertices.len() as u32;

        self.vertices.extend_from_slice(vertices);
        self.indices.extend(indices.iter().map(|i| i + offset));
    }

    pub fn collect(
        renderables: &[Box<dyn Renderable>],
        dims: (f32, f32)
    ) -> impl Iterator<Item = (&Texture, BatchGroup)>
    {
        // TODO: have some better id here than the texture ptr.
        let mut groups: HashMap<*const Texture, (&Texture, BatchGroup)> = HashMap::new();

        for renderable in renderables
        {
            let mesh = renderable.mesh(dims);
            let texture = renderable.texture();

            let indices = mesh.indices.iter().map(|&i| i as u32).collect_vec();

            groups
                .entry(texture as *const Texture)
                .or_insert_with(|| (texture, BatchGroup {
                    vertices: vec![],
                    indices: vec![],
                }))
                .1
                .push(&mesh.vertices, &indices);
        }

        groups.into_values()
    }
}
