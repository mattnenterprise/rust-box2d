use super::distance_proxy::DistanceProxy;
use super::super::common::Sweep;

pub struct TOIInput {
    pub proxy_a: DistanceProxy,
    pub proxy_b: DistanceProxy,
    pub sweep_a: Sweep,
    pub sweep_b: Sweep,
    // defines sweep interval [0, tMax]
    pub t_max: f32
}

impl TOIInput {
    pub fn new() -> TOIInput {
        TOIInput {
            proxy_a: DistanceProxy::new(),
            proxy_b: DistanceProxy::new(),
            sweep_a: Sweep::new(),
            sweep_b: Sweep::new(),
            t_max: 0.0
        }
    }
}
