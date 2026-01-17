use macroquad::color::Color;

pub struct HexGrid {
    pub q_coords: Vec<i32>,
    pub r_coords: Vec<i32>,

    pub screen_x: Vec<f32>,
    pub screen_y: Vec<f32>,

    pub cell_size: u64,

    pub states: Vec<u8>,
    pub next_state: Vec<u8>,

    pub grid_color: Color,
    pub grid_border_color: Color,
}


impl HexGrid {
    pub fn new() -> Self {
    }
}
