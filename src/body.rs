use super::math::Vec2;
use super::shape::shape::Shape;

#[derive(Clone)]
pub enum BodyType {
	StaticBody,
	KinematicBody,
	DynamicBody
}

pub struct BodyDef {
	body_type: BodyType,
	position: Vec2,
	angle: f32,
}

#[derive(Clone)]
pub struct Body {
	pub shape: Shape,
	pub body_type: BodyType,
	pub position: Vec2,
	pub velocity: Vec2,
	pub restitution: f32,
	pub mass: f32
}

impl Body {
	pub fn new(body_def: BodyDef) {

	}
}
