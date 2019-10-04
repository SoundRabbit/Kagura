pub struct AudioWorkletNode {
    pub url: String,
    pub name: String,
}

impl AudioWorkletNode {
    pub fn new(url: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            name: name.into(),
        }
    }
}
