pub struct GainNode {
    pub gain: f64,
}

impl GainNode {
    pub fn new(gain: f64) -> Self {
        Self { gain: gain }
    }
}
