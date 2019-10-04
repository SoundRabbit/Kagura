pub struct AnalyzerNode {
    pub fft_size: Option<u64>,
    pub min_decibels: Option<f64>,
    pub max_decibels: Option<f64>,
    pub smoothing_time_constant: Option<f64>,
}

impl AnalyzerNode {
    pub fn new() -> Self {
        Self {
            fft_size: None,
            min_decibels: None,
            max_decibels: None,
            smoothing_time_constant: None,
        }
    }

    pub fn fft_size(mut self, value: u64) -> Self {
        self.fft_size = Some(value);
        self
    }

    pub fn min_decibels(mut self, value: f64) -> Self {
        self.min_decibels = Some(value);
        self
    }

    pub fn max_decibels(mut self, value: f64) -> Self {
        self.max_decibels = Some(value);
        self
    }

    pub fn smoothing_time_constant(mut self, value: f64) -> Self {
        self.max_decibels = Some(value);
        self
    }
}
