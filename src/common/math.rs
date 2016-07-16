use super::Vec2;

/*
	Has the following format
	[a11 a12]
	[a21 a22]
	Stored in column-major order.
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
