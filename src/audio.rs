pub mod audio_node;

pub enum Connection {}

pub struct AudioNode {
    pub node: AudioNodeType,
    pub id: u128,
}

pub enum AudioNodeType {
    Analyzer(AnalyzerNode),
    AudioWorklet(AudioWorkletNode),
    BiquadFilter(BiquadFilterNode),
    Convolver(ConvolverNode),
    Delay(DelayNode),
    DynamicsCompressor(DynamicsCompressorNode),
    Gain(GainNode),
    Panner(PannerNode),
    WaveShaper(WaveShaperNode),
}

pub struct AnalyzerNode {
    pub fft_size: Option<u64>,
    pub min_decibels: Option<f64>,
    pub max_decibels: Option<f64>,
    pub smoothing_time_constant: Option<f64>,
}

pub struct AudioWorkletNode {
    pub url: String,
    pub name: String,
}

pub struct BiquadFilterNode {
    pub freqency: Option<f64>,
    pub detune: Option<f64>,
    pub q: Option<f64>,
    pub gain: Option<f64>,
}

pub struct ConvolverNode {
    pub buffer: Vec<f64>,
}

pub struct DelayNode {
    pub delay_time: f64,
}
pub struct DynamicsCompressorNode {
    pub attack: Option<f64>,
    pub knee: Option<f64>,
    pub ratio: Option<f64>,
    pub release: Option<f64>,
    pub threshold: Option<f64>,
}

pub struct GainNode {
    pub gain: f64,
}

pub struct PannerNode {
    pub cone_inner_angle: Option<f64>,
    pub cone_outer_angle: Option<f64>,
    pub cone_outer_gain: Option<f64>,
    pub distance_model: Option<DistanceModelType>,
    pub max_distance: Option<f64>,
    pub orientation: Option<(f64, f64, f64)>,
    pub panning_model: Option<PanningModelType>,
    pub position: Option<(f64, f64, f64)>,
    pub ref_distance: Option<(f64, f64, f64)>,
    pub rolloff_factor: Option<f64>,
}

pub struct WaveShaperNode {
    pub curve: Vec<f64>,
    pub oversample: Option<OverSampleType>,
}

pub enum BiquadFilterType {
    Lowpass,
    Highpass,
    Bandpass,
    Lowshelf,
    Highshelf,
    Peaking,
    Notch,
    Allpass,
}

pub enum DistanceModelType {
    Linear,
    Inverse,
    Exponential,
}

pub enum OverSampleType {
    None,
    N2x,
    N4x,
}

pub enum PanningModelType {
    Equalpower,
    Hrtf,
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

impl AudioWorkletNode {
    pub fn new(url: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            name: name.into(),
        }
    }
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

impl ConvolverNode {
    pub fn new(buffer: Vec<f64>) -> Self {
        Self { buffer: buffer }
    }
}

impl DelayNode {
    pub fn new(delay_time: f64) -> Self {
        Self {
            delay_time: delay_time,
        }
    }
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

impl GainNode {
    pub fn new(gain: f64) -> Self {
        Self { gain: gain }
    }
}

impl PannerNode {
    pub fn new() -> Self {
        Self {
            cone_inner_angle: None,
            cone_outer_angle: None,
            cone_outer_gain: None,
            distance_model: None,
            max_distance: None,
            orientation: None,
            panning_model: None,
            position: None,
            ref_distance: None,
            rolloff_factor: None,
        }
    }
}
