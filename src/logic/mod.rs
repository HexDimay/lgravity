use nalgebra::Vector2;

use crate::components::world::World;

pub const G: f32 = 6.67430e-11;
pub const OPT: f32 = G / 1.0; // так надо для ясности.

pub fn update_world(world: &mut World) {
    let readonly_grid = &world.readonly_grid;
    let grid = &mut world.grid;

    for i in 0..grid.cells.len() {
        grid.cells[i].update_density();
        let n = grid.cells[i].neighbor_indexes.clone();

        {
            let cell = &mut grid.cells[i];
            for neighbor_index in n.iter() {
                let n_cell = &readonly_grid.cells[*neighbor_index];
                // обработка всех ускорений
                let v_f = OPT * (cell.mass * n_cell.mass);
                let dv = cell.position - n_cell.position;
                cell.velocity += dv * v_f;
            }
        }

        for neighbor_index in n.iter() {
            let n_cell = &readonly_grid.cells[*neighbor_index];
            // передача массы
            let matching = direction_match(&readonly_grid.cells[i].velocity, &n_cell.velocity);
            if matching > 0.0 {
                let other_density = n_cell.density;
                let cof = grid.cells[i].get_transfer_coefficient(other_density);
                let mass_transfer =
                    grid.cells[i].get_amount_mass_transfer(other_density) * matching;
                let v = grid.cells[i].velocity;
                grid.cells[i].mass -= mass_transfer;
                grid.cells[i].velocity -= v * cof;
                grid.cells[*neighbor_index].mass += mass_transfer;
                grid.cells[*neighbor_index].velocity += v * cof;
            }
        }
    }

    world.readonly_grid = grid.clone();
}

fn direction_match(a: &Vector2<f32>, b: &Vector2<f32>) -> f32 {
    // Нормализуем векторы (получаем единичные векторы)
    let a_normalized = a.normalize();
    let b_normalized = b.normalize();

    // Вычисляем скалярное произведение
    let cos_angle = a_normalized.dot(&b_normalized);

    cos_angle
}
