use macroquad::prelude::*;

mod grid;

use crate::grid::grid::HexGrid;

#[macroquad::main("Cellular Automaton")]
async fn main() {
    loop {
        clear_background(BLACK);

        let mut grid: HexGrid = HexGrid::new(10, 10);

        grid.draw();

        next_frame().await
    }
}
