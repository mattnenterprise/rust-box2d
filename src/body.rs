use super::math::Vec2;
use super::shape::shape::Shape;

#[derive(Clone)]
pub enum BodyType {
	StaticBody,
	KinematicBody,
	DynamicBody
}

pub struct BodyDef {
	pub shape: Shape,
	pub body_type: BodyType,
	pub position: Vec2,
	pub velocity: Vec2,
	pub restitution: f32,
	pub mass: f32
}

#[derive(Clone)]
pub struct Body {
	pub id: usize,
	pub shape: Shape,
	pub body_type: BodyType,
	pub position: Vec2,
	pub velocity: Vec2,
	pub restitution: f32,
	pub mass: f32
}

impl Body {
	pub fn new(id: usize, body_def: BodyDef) -> Body {
		return Body{ id: id, shape: body_def.shape, body_type: body_def.body_type, position: body_def.position, velocity: body_def.velocity, restitution: body_def.restitution, mass: body_def.mass}
	}
	pub fn integrate(&mut self, timeStep: f32) {
		if self.mass <= 0.0 {
			return;
		}
		let inv_mass = 1.0 / self.mass;
		if !inv_mass.is_nan() && !inv_mass.is_infinite() && inv_mass > 0.0 && timeStep > 0.0 {
			//Add gravity to the object;
			let force_accum = Vec2::new(0.0, 10.0);

			let total_accel = force_accum.multiply(inv_mass);
			self.velocity = self.velocity + total_accel.multiply(timeStep);
			self.position = self.position + self.velocity.multiply(timeStep);
		}
	}
}
