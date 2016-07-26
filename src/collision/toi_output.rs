pub enum TOIOutputState {
    Unknown,
    Failed,
    Overlapped,
    Touching,
    Separated
}

pub struct TOIOutput {
    pub state: TOIOutputState,
    pub t: f32
}

impl TOIOutput {
    pub fn new(t: f32) -> TOIOutput {
        TOIOutput {
            state: TOIOutputState::Unknown,
            t: t
        }
    }
}
