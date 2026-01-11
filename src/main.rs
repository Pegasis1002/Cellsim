use macroquad::prelude::*;

#[macroquad::main("Cellular Automaton")]
async fn main() {
    loop {
        clear_background(BLACK);

        next_frame().await
    }
}
