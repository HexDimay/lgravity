use bytemuck::{Pod, Zeroable};

use crate::components::world::World;

pub const SIZE_RENDER_CELLS: f32 = 10.0;

#[repr(C)]
#[derive(Debug, Clone, Copy, Zeroable, Pod)]
pub struct Vertex {
    pub position: [f32; 3],
}

impl Vertex {
    pub fn new(position: [f32; 3]) -> Self {
        Self { position }
    }
}

#[derive(Debug)]
pub struct RenderWorld {
    vertex_data: Vec<Vertex>,
}

impl RenderWorld {
    /// `count_data` это есть длина нашей сетки `n*m`.
    pub fn new(count_data: usize) -> Self {
        Self {
            vertex_data: Vec::with_capacity(count_data),
        }
    }

    fn create_vertices(x: f32, y: f32) -> [Vertex; 6] {
        [
            Vertex::new([x, y, 0.0]),
            Vertex::new([x, y + SIZE_RENDER_CELLS, 0.0]),
            Vertex::new([x + SIZE_RENDER_CELLS, y + SIZE_RENDER_CELLS, 0.0]),
            
            Vertex::new([x, y, 0.0]),
            Vertex::new([x + SIZE_RENDER_CELLS, y, 0.0]),
            Vertex::new([x + SIZE_RENDER_CELLS, y + SIZE_RENDER_CELLS, 0.0]),
        ]
    }

    pub fn init_data_world(&mut self, world: &World) {
        for y in 0..world.height() {
            for x in 0..world.width() {
                let (x, y) = (x as f32 * SIZE_RENDER_CELLS, y as f32 * SIZE_RENDER_CELLS);
                self.vertex_data.extend(Self::create_vertices(x, y));
            }
        }
    }
}

