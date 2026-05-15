use rand::distributions::uniform::SampleRange;

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

    pub fn randomize_mass<R>(&mut self, range: R)
    where
        R: SampleRange<f32> + Clone,
    {
        self.grid.randomize_mass(range);
    }

    pub fn get_grid(&self) -> &Grid {
        &self.grid
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}
