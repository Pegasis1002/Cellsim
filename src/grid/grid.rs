pub struct HexGrid {
    q_coords: Vec<i32>,
    r_coords: Vec<i32>,
    width: usize,
    height: usize,
    states: Vec<u8>,
    next_state: Vec<u8>,
}


impl HexGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        let mut q_coords = Vec::with_capacity(size);
        let mut r_coords = Vec::with_capacity(size);

        for r in 0..height {
            for q in 0..width {
                q_coords.push(q as i32);
                r_coords.push(r as i32);
            }
        }

        return Self {
            q_coords,
            r_coords,
            width,
            height,
            states: vec![0x0; size],
            next_state: vec![0; size],
        }
    }
}
