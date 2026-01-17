use macroquad::shapes::draw_hexagon;

use crate::grid::grid::HexGrid;

impl HexGrid {
    pub fn draw(grid: &HexGrid) {
        for q in &grid.q_coords {
            for r in &grid.r_coords {
                let qc: f32 = (q * (grid.cell_size as i32)) as f32;
                let rc: f32 = (r * (grid.cell_size as i32)) as f32;
                let rel_cell_size = 2.0/(grid.cell_size as f32);

                draw_hexagon(qc, rc, rel_cell_size, rel_cell_size/5.0, true, grid.grid_border_color, grid.grid_color);
            }
        }
    }
}
