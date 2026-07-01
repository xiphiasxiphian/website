use crate::renderer::vertex::Vertex;

struct BatchGroup
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
}
