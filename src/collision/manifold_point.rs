use super::super::common::Vec2;
use super::contact_id::ContactID;

/// A manifold point is a contact point belonging to a contact
/// manifold. It holds details related to the geometry and dynamics
/// of the contact points.
/// The local point usage depends on the manifold type:
/// -e_circles: the local center of circleB
/// -e_faceA: the local center of cirlceB or the clip point of polygonB
/// -e_faceB: the clip point of polygonA
/// This structure is stored across time steps, so we keep it small.
/// Note: the impulses are used for internal caching and may not
/// provide reliable contact forces, especially for high speed collisions.
pub struct ManifoldPoint {
    /// Usage depends on manifold type
    pub local_point: Vec2,
    /// the non-penetration impulse
    pub normal_impulse: f32,
    /// the friction impulse
    pub tangent_impulse: f32,
    /// uniquely identifies a contact point between two shapes
    pub id: ContactID
}

impl ManifoldPoint {
    fn new() -> ManifoldPoint {
        ManifoldPoint {
            local_point: Vec2::zero(),
            normal_impulse: 0.0,
            tangent_impulse: 0.0,
            id: ContactID::new()
        }
    }
}
