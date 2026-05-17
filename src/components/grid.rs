use rand::{Rng, distributions::uniform::SampleRange};

use crate::components::cell::Cell;

pub const PADDING_VIEW: isize = 1;

#[derive(Debug, Clone)]
pub struct Grid {
    width: usize,
    height: usize,
    pub cells: Vec<Cell>,
}

impl Grid {
    /// Создаёт сетку, со сторонами `n*m`.
    pub fn new(width: usize, height: usize, default_mass: f32) -> Self {
        Self {
            width,
            height,
            cells: vec![Cell::new(default_mass); width * height],
        }
    }

    /// Случайно распределяет массы ячеек в сетке в заданном диапазоне.
    pub fn randomize_mass<R>(&mut self, range: R)
    where
        R: SampleRange<f32> + Clone,
    {
        for cell in &mut self.cells {
            cell.mass = rand::thread_rng().gen_range(range.clone());
        }
    }

    pub fn update_cell_positions(&mut self) {
        let width = self.width;
        for (idx, cell) in self.cells.iter_mut().enumerate() {
            cell.position.x = (idx % width) as f32;
            cell.position.y = (idx / width) as f32;
        }
    }

    pub fn build_neighbor_indexes(&mut self) {
        let len = self.cells.len();
        for i in 0..len {
            let position = self.cells[i].position;
            let mut pos_neighbors = Vec::new();
            for x in -PADDING_VIEW..=PADDING_VIEW {
                for y in -PADDING_VIEW..=PADDING_VIEW {
                    if x != 0 && y != 0 {
                        let (x, y) = (position.x as isize + x, position.y as isize + y);
                        if self.valid_position(x, y) {
                            pos_neighbors.push((x + (y * self.width as isize)) as usize);
                        }
                    }
                }
            }

            self.cells[i].neighbor_indexes = pos_neighbors;
        }
    }

    fn valid_position(&self, x: isize, y: isize) -> bool {
        let (x, y) = (x as usize, y as usize);
        x < self.width && y < self.height && x >= 0 && y >= 0
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<&Cell> {
        self.cells.get(x + y * self.width)
    }
}
