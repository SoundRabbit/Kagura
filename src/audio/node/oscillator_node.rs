pub struct OscillatorNode {
    pub freqency: f64
}

impl OscillatorNode {
    pub fn new(freqency: f64) -> Self {
        Self { freqency: freqency }
    }
}