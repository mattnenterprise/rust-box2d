use super::super::common::Vec2;

#[derive(Clone)]
pub enum Shape {
     CircleShape{center: Vec2, radius: f32},
     LineShape{point1: Vec2, point2: Vec2},
     ChainLineShape{points: Vec<Vec2>},
     PolygonShape{points: Vec<Vec2>},
}

pub enum ShapeType {
    Circle,
    Edge,
    Polygon,
    Chain
}
