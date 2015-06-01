use super::super::math::Vec2;

#[derive(Clone)]
pub enum Shape {
     CircleShape{center: Vec2, radius: f32},
     LineShape{point1: Vec2, point2: Vec2}
}
