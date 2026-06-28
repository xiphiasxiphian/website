use crate::renderer::vertex::Vertex;

pub struct Mesh
{
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

impl Mesh
{
    pub fn quad(x: f32, y: f32, w: f32, h: f32, dims: (f32, f32)) -> Self
    {
        debug_assert!(
            x >= 0.0 && x + w <= dims.0,
            "quad x out of bounds: {} + {} > {}",
            x,
            w,
            dims.0
        );
        debug_assert!(
            y >= 0.0 && y + h <= dims.1,
            "quad y out of bounds: {} + {} > {}",
            y,
            h,
            dims.1
        );

        let to_ndc_x = |px: f32| (px / dims.0) * 2.0 - 1.0;
        let to_ndc_y = |py: f32| 1.0 - (py / dims.1) * 2.0;

        let x0 = to_ndc_x(x);
        let y0 = to_ndc_y(y);
        let x1 = to_ndc_x(x + w);
        let y1 = to_ndc_y(y + h);

        Self {
            vertices: vec![
                Vertex {
                    position: [x0, y0],
                    texture_coords: [0.0, 0.0],
                }, // top-left
                Vertex {
                    position: [x1, y0],
                    texture_coords: [1.0, 0.0],
                }, // top-right
                Vertex {
                    position: [x1, y1],
                    texture_coords: [1.0, 1.0],
                }, // bottom-right
                Vertex {
                    position: [x0, y1],
                    texture_coords: [0.0, 1.0],
                }, // bottom-left
            ],
            indices: vec![0, 2, 1, 0, 3, 2],
        }
    }
}
