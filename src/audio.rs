pub mod audio_node;

use wasm_bindgen::prelude::*;

pub enum Connection {}

pub struct AudioNode {
    node: AudioNodeType,
    id: u128,
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
    fft_size: Option<u32>,
    min_decibels: Option<f64>,
    max_decibels: Option<f64>,
    smoothing_time_constant: Option<f64>,
}

pub struct AudioWorkletNode {
    url: String,
    name: String,
}

pub struct BiquadFilterNode {
    freqency: Option<f64>,
    detune: Option<f64>,
    q: Option<f64>,
    gain: Option<f64>,
}

pub struct ConvolverNode {
    buffer: Vec<f32>,
}

pub struct DelayNode {
    delay_time: Option<f64>,
}
pub struct DynamicsCompressorNode {
    attack: Option<f32>,
    knee: Option<f32>,
    ratio: Option<f32>,
    release: Option<f32>,
    threshold: Option<f32>,
}

pub struct GainNode {
    gain: f32,
}

pub struct PannerNode {
    cone_inner_angle: Option<f32>,
    cone_outer_angle: Option<f32>,
    cone_outer_gain: Option<f32>,
    distance_model: Option<DistanceModelType>,
    max_distance: Option<f32>,
    orientation: Option<(f32, f32, f32)>,
    panning_model: Option<PanningModelType>,
    position: Option<(f32, f32, f32)>,
    ref_distance: Option<(f32, f32, f32)>,
    rolloff_factor: Option<f32>,
}

pub struct WaveShaperNode {
    curve: Vec<f32>,
    oversample: Option<OverSampleType>
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

pub enum OverSampleType{
    None,
    N2x,
    N4x,
}

pub enum PanningModelType {
    Equalpower,
    Hrtf,
}
