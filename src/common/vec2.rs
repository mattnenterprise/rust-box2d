use std::ops::{Add, AddAssign, Sub, SubAssign, Neg, Mul, MulAssign};

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

    pub fn set_zero(&mut self) {
		self.set(0.0, 0.0)
	}

	pub fn set(&mut self, x: f32, y: f32) {
		self.x = x;
		self.y = y;
	}

	/// Perform the dot product of two vectors.
	pub fn dot(a: Vec2, b: Vec2) -> f32 {
		return a.x * b.x + a.y * b.y;
	}

	pub fn length(self) -> f32 {
		return f32::sqrt(self.x * self.x + self.y * self.y);
	}

	pub fn divide(self, n: f32) -> Vec2 {
		return Vec2 {x: self.x / n, y: self.y / n};
	}

	/// Perform the cross product on two vectors. In 2D this produces a scalar.
	pub fn cross_vec(a: Vec2, b: Vec2) -> f32 {
		a.x * b.y - a.y * b.x
	}

	/// Perform a cross product on a vector and a scalar. In 2D this produces
	/// a vector.
	pub fn cross_vec_scalar(a: Vec2, s: f32) -> Vec2 {
		Vec2::new(s * a.y, -s * a.x)
	}

	/// Perform the cross product on a scalar and a vector. In 2D this produces
	/// a vector.
	pub fn cross_scalar_vec(s: f32, a: Vec2) -> Vec2 {
		Vec2::new(-s * a.y, s * a.x)
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
		Vec2::new(self.x + _rhs.x, self.y + _rhs.y)
	}
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, _rhs: Vec2) {
        self.x += _rhs.x;
		self.y += _rhs.y;
    }
}

impl Sub for Vec2 {
	type Output = Vec2;

	fn sub(self, _rhs: Vec2) -> Vec2 {
		Vec2::new(self.x - _rhs.x, self.y - _rhs.y)
	}
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, _rhs: Vec2) {
		self.x -= _rhs.x;
		self.y -= _rhs.y;
    }
}

impl Mul<f32> for Vec2 {
	type Output = Vec2;

    fn mul(self, _rhs: f32) -> Vec2 {
		Vec2::new(self.x * _rhs, self.y * _rhs)
    }
}

impl Mul<Vec2> for f32 {
	type Output = Vec2;

	fn mul(self, _rhs: Vec2) -> Vec2 {
		Vec2::new(_rhs.x * self, _rhs.y * self)
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, _rhs: f32) {
        self.x *= _rhs;
		self.y *= _rhs;
    }
}

impl Neg for Vec2 {
	type Output = Vec2;

	fn neg(self) -> Vec2 {
		Vec2::new(-self.x, -self.y)
    }
}
