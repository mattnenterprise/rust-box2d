use super::super::common::Vec2;
use super::super::common::Transform;
use super::super::common::settings::MAX_MANIFOLD_POINTS;
use super::manifold::Manifold;

/// This is used to compute the current state of a contact manifold.
pub struct WorldManifold {
    /// world vector pointing from A to B
    pub normal: Vec2,
    /// world contact point (point of intersection)
    pub points: [Vec2; MAX_MANIFOLD_POINTS as usize],
    /// a negative value indicates overlap, in meters
    pub separations: [f32; MAX_MANIFOLD_POINTS as usize]
}

impl WorldManifold {
    pub fn new() -> WorldManifold {
        WorldManifold {
            normal: Vec2::zero(),
            points: [Vec2::zero(); MAX_MANIFOLD_POINTS as usize],
            separations: [0.0; MAX_MANIFOLD_POINTS as usize]
        }
    }

    /// Evaluate the manifold with supplied transforms. This assumes
	/// modest motion from the original state. This does not change the
	/// point count, impulses, etc. The radii must come from the shapes
	/// that generated the manifold.
    pub fn initialize(manifold: Manifold, xf_a: Transform, radius_a: f32, xf_b: Transform, radius_b: f32)  {
        // TODO implement
    }
}
