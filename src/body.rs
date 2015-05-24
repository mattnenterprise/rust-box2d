use super::math::Vec2;
use super::shape::shape::Shape;

pub enum BodyType {
	StaticBody,
	KinematicBody,
	DynamicBody
}

pub struct BodyDef {
	body_type: BodyType,
	position: Vec2,
	angle: f32,
	gravity_scale: f32
}

pub struct Body {
	body_type: BodyType,
	position: Vec2,
	rotation: f32,
	velocity: Vec2,
	angularVelocity: f32,
	force: Vec2,
	torque: f32,
	mass: f32,
	inv_mass: f32
}

impl Body {
	pub fn new(body_def: BodyDef) {

	}
}
