use macroquad::prelude::*;


pub fn newCam() -> Camera2D {
    let cam = Camera2D {
        zoom: Vec2 { x: 1.0, y: 1.0 },
        target: Vec2 { x: 1.0, y: 0.0 },
        offset: Vec2 { x: 0.0, y: 0.0 },
        ..Default::default()
    };

    return cam;
}
