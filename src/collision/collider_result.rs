use super::super::manifold::Manifold;

pub struct ColliderResult {
    pub manifold: Option<Manifold>,
    pub is_colliding: bool
}

impl ColliderResult {
    pub fn new(manifold: Option<Manifold>, is_colliding: bool) -> ColliderResult {
        return ColliderResult{manifold: manifold, is_colliding: is_colliding};
    }

    pub fn new_empty_false() -> ColliderResult {
        return ColliderResult{manifold: None, is_colliding: false};
    }
}
