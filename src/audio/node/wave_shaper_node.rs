pub struct WaveShaperNode {
    pub curve: Vec<f64>,
    pub oversample: Option<OverSampleType>,
}

pub enum OverSampleType {
    None,
    N2x,
    N4x,
}

impl WaveShaperNode {
    pub fn new(curve: Vec<f64>) -> Self {
        Self {
            curve: curve,
            oversample: None,
        }
    }

    pub fn oversample(mut self, value: OverSampleType) -> Self {
        self.oversample = Some(value);
        self
    }
}
