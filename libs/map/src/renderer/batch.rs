use wgpu::{Buffer, BufferDescriptor, BufferUsages, Device, Queue};

use crate::renderer::vertex::Vertex;

pub struct BatchGroup
{
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl BatchGroup
{
    pub fn push(&mut self, vertices: &[Vertex], indices: &[u32])
    {
        let offset = self.vertices.len() as u32;

        self.vertices.extend_from_slice(vertices);
        self.indices.extend(indices.iter().map(|i| i + offset));
    }
}

pub struct BufferPool
{
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    vertex_capacity: usize,
    index_capacity: usize,
}

impl BufferPool
{
    pub fn new(
        device: &Device,
        max_vertices: usize,
        max_indices: usize,
    ) -> Self
    {
        let vertex_capacity = max_vertices * size_of::<Vertex>();
        let index_capacity = max_indices * size_of::<u32>();

        let vertex_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("persistent_vertex_buffer"),
            size: vertex_capacity as u64,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let index_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("persistent_index_buffer"),
            size: index_capacity as u64,
            usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            vertex_buffer,
            index_buffer,
            vertex_capacity,
            index_capacity,
        }
    }

    pub fn write(
        &self,
        queue: &Queue,
        vertices: &[Vertex],
        indices: &[u32]
    )
    {
        let vb = bytemuck::cast_slice(vertices);
        let ib = bytemuck::cast_slice(indices);

        assert!(vb.len() <= self.vertex_capacity, "Vertex buffer overflow");
        assert!(ib.len() <= self.index_capacity,  "Index buffer overflow");

        queue.write_buffer(&self.vertex_buffer, 0, vb);
        queue.write_buffer(&self.index_buffer,  0, ib);
    }
}
