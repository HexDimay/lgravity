use crate::components::cell::Cell;

#[derive(Debug)]
pub struct Grid {
    pub cells: Vec<Cell>
}

impl Grid {
    /// Создаёт сетку, со сторонами `n*m`.
    pub fn new(n: usize, m: usize, default_mass: f32) -> Self {
        Self {
            cells: vec![Cell::new(default_mass); n * m],
        }
    }
}
