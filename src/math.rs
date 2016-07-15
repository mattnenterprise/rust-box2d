use std::ops::{Add, Sub, Neg};

#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
	pub x: f32,
	pub y: f32
}

impl Vec2 {
	pub fn new(x: f32, y: f32) -> Vec2 {
		return Vec2 {x: x, y: y}
	}

	pub fn zero() -> Vec2 {
		return Vec2 {x: 0.0, y: 0.0}
	}

	pub fn set(&mut self, x: f32, y: f32) {
		self.x = x;
		self.y = y;
	}

	pub fn set_zero(&mut self) {
		self.set(0.0, 0.0)
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

	pub fn cross(self, o: Vec2) -> f32 {
		return self.x * o.y - self.y * o.x;
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

impl Neg for Vec2 {
	type Output = Vec2;

	fn neg(self) -> Vec2 {
		Vec2::new(-self.x, -self.y)
    }
}

/*
	Has the following format
	[a11 a12]
	[a21 a22]
*/
pub struct Mat22 {
	pub c1: Vec2,
	pub c2: Vec2
}

impl Mat22 {
	pub fn new(a11: f32, a12: f32, a21: f32, a22: f32) -> Mat22 {
		return Mat22 {
			c1: Vec2::new(a11, a21),
			c2: Vec2::new(a12, a22)
		}
	}
}
