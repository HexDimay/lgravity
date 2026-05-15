use rand::{Rng, distributions::uniform::SampleRange};

use crate::components::cell::Cell;

#[derive(Debug)]
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

    pub fn get_cell(&self, x: usize, y: usize, width: usize) -> Option<&Cell> {
        self.cells.get(x + y * width)
    }
}
