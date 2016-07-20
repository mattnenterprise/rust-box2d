use super::super::common::Vec2;

pub struct Position {
    pub c: Vec2,
    pub a: f32
}

impl Position {
    pub fn new() -> Position {
        Position {
            c: Vec2::zero(),
            a: 0.0
        }
    }
}
