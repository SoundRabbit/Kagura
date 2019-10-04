pub struct DynamicsCompressorNode {
    pub attack: Option<f64>,
    pub knee: Option<f64>,
    pub ratio: Option<f64>,
    pub release: Option<f64>,
    pub threshold: Option<f64>,
}

impl DynamicsCompressorNode {
    pub fn new() -> Self {
        Self {
            attack: None,
            knee: None,
            ratio: None,
            release: None,
            threshold: None,
        }
    }

    pub fn attack(mut self, value: f64) -> Self {
        self.attack = Some(value);
        self
    }

    pub fn knee(mut self, value: f64) -> Self {
        self.knee = Some(value);
        self
    }

    pub fn ratio(mut self, value: f64) -> Self {
        self.ratio = Some(value);
        self
    }

    pub fn release(mut self, value: f64) -> Self {
        self.release = Some(value);
        self
    }

    pub fn threshold(mut self, value: f64) -> Self {
        self.threshold = Some(value);
        self
    }
}
