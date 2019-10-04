pub struct DelayNode {
    pub delay_time: f64,
}

impl DelayNode {
    pub fn new(delay_time: f64) -> Self {
        Self {
            delay_time: delay_time,
        }
    }
}
