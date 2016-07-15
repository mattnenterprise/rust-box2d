use time::precise_time_ns;

pub struct Timer {
    start: u64
}

impl Timer {
    fn new() -> Timer {
        let mut t = Timer{
            start: 0,
        };
        t.reset();
        t
    }

    pub fn reset(&mut self) {
        self.start = precise_time_ns();
    }

    pub fn get_milliseconds(self) -> f32 {
        // TODO Better way to do this ?
        ((precise_time_ns() - self.start) as f64 / 1000000.0) as f32
    }
}
