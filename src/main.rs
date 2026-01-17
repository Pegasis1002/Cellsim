use macroquad::prelude::*;

mod grid;
mod camera;

use crate::grid::grid::HexGrid;
use crate::camera::camera::newCam;

#[macroquad::main("Cellular Automaton")]
async fn main() {
    loop {
        clear_background(BLACK);
        let mut cam = newCam();
        set_camera(&cam);

        let mut grid: HexGrid = HexGrid::new(20, 20);

        HexGrid::draw(&grid);

        next_frame().await
    }
}
