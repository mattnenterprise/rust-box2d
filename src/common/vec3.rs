use std::ops::{Neg, AddAssign, SubAssign, MulAssign};

pub struct Vec3 {
    pub x: f32,
	pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
		return Vec3 {x: x, y: y, z: z}
	}

    pub fn set_zero(&mut self) {
		self.set(0.0, 0.0, 0.0)
	}

	pub fn set(&mut self, x: f32, y: f32, z: f32) {
		self.x = x;
		self.y = y;
        self.z = z;
	}

    /// Peform the dot product on two vectors
    pub fn dot(a: Vec3, b: Vec3) -> f32 {
        a.x * b.x +a.y * b.y + a.z * b.z
    }

    /// Perform the cross product on two vectors.
    pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
        Vec3::new(a.y * b.z - a.z * b.y, a.z * b.x - a.x * b.z, a.x * b.y - a.y * b.x)
    }
}

impl Neg for Vec3 {
	type Output = Vec3;

	fn neg(self) -> Vec3 {
		Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, _rhs: Vec3) {
        self.x += _rhs.x;
		self.y += _rhs.y;
        self.z += _rhs.z;
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, _rhs: Vec3) {
		self.x -= _rhs.x;
		self.y -= _rhs.y;
        self.z -= _rhs.z;
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, _rhs: f32) {
        self.x *= _rhs;
		self.y *= _rhs;
        self.z *= _rhs;
    }
}
