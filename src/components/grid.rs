use rand::{Rng, distributions::uniform::SampleRange};

use crate::components::cell::Cell;

#[derive(Debug, Clone)]
pub struct Grid {
    pub cells: Vec<Cell>,
}

impl Grid {
    /// Создаёт сетку, со сторонами `n*m`.
    pub fn new(n: usize, m: usize, default_mass: f32) -> Self {
        Self {
            cells: vec![Cell::new(default_mass); n * m],
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

    pub fn update_cell_positions(&mut self, width: usize) {
        for (idx, cell) in self.cells.iter_mut().enumerate() {
            cell.position.x = (idx % width) as f32;
            cell.position.y = (idx / width) as f32;
        }
    }

    pub fn build_neighbor_indexes(&mut self, width: usize) {
        let len = self.cells.len();
        for (idx, cell) in self.cells.iter_mut().enumerate() {
            let pos_cell = ((idx % width) as isize, (idx / width) as isize);
            let mut pos_neighbors = Vec::new();
            for x in -1..=1 {
                for y in -1..=1 {
                    if x != 0 && y != 0 {
                        pos_neighbors.push((x, y));
                    }
                }
            }
            let pos_neighbors = pos_neighbors
                .iter()
                .map(|(x, y)| (pos_cell.0 + x, pos_cell.1 + y))
                .collect::<Vec<_>>();

            let valid_neighbors = pos_neighbors
                .iter()
                .map(|(x, y)| {
                    if Self::valid_position(len, *x, *y, width)
                        && Self::valid_index(len, *x + (*y * width as isize))
                    {
                        return Some((*x + (*y * width as isize)) as usize);
                    }

                    None
                })
                .flatten()
                .collect();

            cell.neighbor_indexes = valid_neighbors;
        }
    }

    fn valid_index(len: usize, index: isize) -> bool {
        index < len as isize && index >= 0
    }

    fn valid_position(len: usize, x: isize, y: isize, width: usize) -> bool {
        x < width as isize && y < len as isize / width as isize
    }

    pub fn get_cell(&self, x: usize, y: usize, width: usize) -> Option<&Cell> {
        self.cells.get(x + y * width)
    }
}
