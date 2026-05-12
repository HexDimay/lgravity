use nalgebra::Vector2;

/// m^2
pub const CELL_SIZE: f32 = 1.0;
pub const TRANSMISSION_RATIO_OF_VELOCITY: f32 = 10.0;

#[derive(Debug, Clone)]
pub struct Cell {
    pub mass: f32,
    pub velocity: Vector2<f32>,
    pub density: f32,
}

impl Cell {
    pub fn new(mass: f32) -> Self {
        Self {
            mass,
            velocity: Vector2::new(0.0, 0.0),
            density: mass / CELL_SIZE,
        }
    }

    pub const fn update_density(&mut self) {
        self.density = self.mass / CELL_SIZE;
    }

    fn get_len_vector(v: &Vector2<f32>) -> f32 {
        (v.x.powf(2.0) + v.y.powf(2.0)).sqrt()
    }

    /// Получение коэффициента передачи массы, зависящего от плотности другой ячейки. \
    /// Чем больше плотность, тем меньше коэффициент.
    #[inline]
    pub fn get_transfer_coefficient(&self, density_other_cell: f32) -> f32 {
        1.0 / density_other_cell.sqrt() // TODO: упростить выражение потом
    }

    /// Получение количества массы, передаваемого в другую ячейку. \
    /// Зависит от массы, скорости и плотности другой ячейки.
    #[inline]
    pub fn get_amount_mass_transfer(&self, density_other_cell: f32) -> f32 {
        self.mass
            * (Self::get_len_vector(&self.velocity) / TRANSMISSION_RATIO_OF_VELOCITY
                * self.get_transfer_coefficient(density_other_cell))
    }
}
