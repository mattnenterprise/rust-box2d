enum ContactIDType {
    VERTEX = 0,
    FACE = 1
}

/// Contact ids to facilitate warm starting. Note: the ContactFeature struct is just embedded in here.
/// A union is used for this type in Box2D. We just embed it.
pub struct ContactID {
    pub index_a: u8,
    pub index_b: u8,
    pub type_a: u8,
    pub type_b: u8,
}

impl ContactID {
    pub fn new() -> ContactID {
        ContactID {
            index_a: 0,
            index_b: 0,
            type_a: 0,
            type_b: 0
        }
    }

    pub fn key(self) -> u32 {
        (self.index_a as u32) << 24 | (self.index_b as u32) << 16 | (self.type_a as u32) << 8 | self.type_b as u32
    }
}
