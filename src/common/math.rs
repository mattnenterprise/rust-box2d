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
