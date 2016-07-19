/// Used to warm start Distance.
/// Set count to zero on first call.
pub struct SimplexCache {
    pub metric: f32,
    pub count: i16,
    pub index_a: [u8; 3],
    pub index_b: [u8; 3]
}

impl SimplexCache {
    pub fn new() -> SimplexCache {
        SimplexCache {
            metric: 0.0,
            count: 0,
            index_a: [0; 3], // TODO not for sure if this should be 0
            index_b: [0; 3]  // TODO not for sure if this should be 0
        }
    }
}
