use rand::distributions::uniform::SampleRange;

use crate::components::grid::Grid;

pub const DEAFULT_MASS_OF_CELL: f32 = 1.0;

#[derive(Debug)]
pub struct World {
    width: usize,
    height: usize,
    pub readonly_grid: Grid,
    pub grid: Grid,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        let mut grid = Grid::new(width, height, DEAFULT_MASS_OF_CELL);
        grid.update_cell_positions(width);
        grid.build_neighbor_indexes(width);
        Self {
            width,
            height,
            readonly_grid: grid.clone(),
            grid: grid,
        }
    }

    pub fn randomize_mass<R>(&mut self, range: R)
    where
        R: SampleRange<f32> + Clone,
    {
        self.grid.randomize_mass(range);
        self.readonly_grid = self.grid.clone();
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
