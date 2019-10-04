pub struct BiquadFilterNode {
    pub freqency: Option<f64>,
    pub detune: Option<f64>,
    pub q: Option<f64>,
    pub gain: Option<f64>,
}

impl BiquadFilterNode {
    pub fn new() -> Self {
        Self {
            freqency: None,
            detune: None,
            q: None,
            gain: None,
        }
    }

    pub fn freqency(mut self, value: f64) -> Self {
        self.freqency = Some(value);
        self
    }

    pub fn detune(mut self, value: f64) -> Self {
        self.detune = Some(value);
        self
    }

    pub fn q(mut self, value: f64) -> Self {
        self.q = Some(value);
        self
    }

    pub fn gain(mut self, value: f64) -> Self {
        self.gain = Some(value);
        self
    }
}
