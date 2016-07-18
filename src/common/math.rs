use super::Vec2;
use super::Rot;
use super::Transform;

// This function is used to ensure that a floating point number of non a NaN or infinity.
pub fn is_valid(x: f32) -> bool {
    !(x.is_nan() || x.is_infinite())
}

/// "Next Largest Power of 2
/// Given a binary integer value x, the next largest power of 2 can be computed by a SWAR algorithm
/// that recursively "folds" the upper bits into the lower bits. This process yields a bit vector with
/// the same most significant 1 as x, but all 1's below it. Adding 1 to that value yields the next
/// largest power of 2. For a 32-bit value:"
pub fn next_power_of_two(mut x: u32) -> u32 {
    x |= x >> 1;
    x |= x >> 2;
	x |= x >> 4;
	x |= x >> 8;
	x |= x >> 16;
    return x + 1;
}

pub fn is_power_of_two(x: u32) -> bool {
    x > 0 && (x & (x - 1)) == 0
}

/// Get the minumum vector
pub fn min_vec(a: Vec2, b: Vec2) -> Vec2 {
    Vec2 {
        x: a.x.min(b.x),
        y: a.y.min(b.y)
    }
}

/// Get the maximum vector
pub fn max_vec(a: Vec2, b: Vec2) -> Vec2 {
    Vec2 {
        x: a.x.max(b.x),
        y: a.y.max(b.y)
    }
}

/// Rotate a vector
pub fn mul_rot_vec2(q: Rot, v: Vec2) -> Vec2 {
    Vec2::new(q.c * v.x - q.s * v.y, q.s * v.x + q.c * v.y)
}

/// Inverse rotate a vector
pub fn mul_rot_vec2_inverse(q: Rot, v: Vec2) -> Vec2 {
    Vec2::new(q.c * v.x + q.s * v.y, -q.s * v.x + q.c * v.y)
}

pub fn mul_transform_vec2(t: Transform, v: Vec2) -> Vec2 {
	let x = (t.q.c * v.x - t.q.s * v.y) + t.p.x;
	let y = (t.q.s * v.x + t.q.c * v.y) + t.p.y;

    Vec2::new(x, y)
}

pub fn mul_transform_vec2_inverse(t: Transform, v: Vec2) -> Vec2 {
	let px = v.x - t.p.x;
	let py = v.y - t.p.y;
	let x = t.q.c * px + t.q.s * py;
	let y = -t.q.s * px + t.q.c * py;

    Vec2::new(x, y)
}
