use crate::native;
use super::Node;

pub struct Analyzer {

}

pub struct AudioBufferSource {

}

pub struct BiquadFilter {

}

pub struct ConstantSource {

}

pub struct Convolver {

}

pub struct Delay {

}

pub struct DynamicsCompressor {

}

pub struct Gain {

}

pub struct IIRFilter{

}

pub struct MediaElementAudioSource{

}

pub struct MediaStreamAudioDestination{

}

pub struct MediaStreamAudioSource {

}

pub struct Oscillator {
    freqency: f32,
    detune: f32,
    type_: OscillatorType,
}

pub struct Panner {

}

pub struct ScriptProcessor {
    on_audioprocess: Box<FnMut(native::Event)>
}

pub struct WaveShaper {

}

pub enum OscillatorType {
    Sine,
    Square,
    Sawtooth,
    Triangle
}