use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
	pub x: f32,
	pub y: f32
}

impl Vec2 {
	pub fn new(x: f32, y: f32) -> Vec2 {
		return Vec2 {x: x, y: y}
	}

	pub fn new_zero() -> Vec2 {
		return Vec2 {x: 0.0, y: 0.0}
	}

	pub fn set(&mut self, x: f32, y: f32) {
		self.x = x;
		self.y = y;
	}

	pub fn dot(&mut self, o: Vec2) -> f32 {
		return self.x * o.x + self.y * o.y;
	}

	pub fn length(self) -> f32 {
		return f32::sqrt(self.x * self.x + self.y * self.y);
	}

	pub fn multiply(self, n: f32) -> Vec2 {
		return Vec2 {x: self.x * n, y: self.y * n};
	}

	pub fn divide(self, n: f32) -> Vec2 {
		return Vec2 {x: self.x / n, y: self.y / n};
	}

	pub fn normal(self) -> Vec2 {
		let length = self.length();
		if length == 0.0 {
			panic!("Cannot find normal when length is zero");
		}
		return self.divide(length);
	}
}

impl Add for Vec2 {
	type Output = Vec2;

	fn add(self, _rhs: Vec2) -> Vec2 {
		return Vec2::new(self.x + _rhs.x, self.y + _rhs.y);
	}
}

impl Sub for Vec2 {
	type Output = Vec2;

	fn sub(self, _rhs: Vec2) -> Vec2 {
		return Vec2::new(self.x - _rhs.x, self.y - _rhs.y);
	}
}
