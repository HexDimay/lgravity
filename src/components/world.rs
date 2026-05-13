use crate::components::grid::Grid;

pub const DEAFULT_MASS_OF_CELL: f32 = 1.0;

#[derive(Debug)]
pub struct World {
    width: usize,
    height: usize,
    grid: Grid,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            grid: Grid::new(width, height, DEAFULT_MASS_OF_CELL),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}
