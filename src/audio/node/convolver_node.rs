pub struct ConvolverNode {
    pub buffer: Vec<f64>,
}

impl ConvolverNode {
    pub fn new(buffer: Vec<f64>) -> Self {
        Self { buffer: buffer }
    }
}
