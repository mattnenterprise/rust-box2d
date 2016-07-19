use super::distance_proxy::DistanceProxy;
use super::super::common::Transform;

/// Input for b2Distance.
/// You have to option to use the shape radii
/// in the computation. Even
pub struct DistanceInput {
    pub proxy_a: DistanceProxy,
    pub proxy_b: DistanceProxy,
    pub transform_a: Transform,
    pub transform_b: Transform,
    pub use_radii: bool
}

impl DistanceInput {
    pub fn new() -> DistanceInput {
        DistanceInput {
            proxy_a: DistanceProxy::new(),
            proxy_b: DistanceProxy::new(),
            transform_a: Transform::new(),
            transform_b: Transform::new(),
            use_radii: false
        }
    }
}
