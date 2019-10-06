mod analyser_node;
mod audio_worklet_node;
mod biquad_filter_node;
mod convolver_node;
mod delay_node;
mod dynamics_compressor_node;
mod gain_node;
mod media_stream_destination_node;
mod oscillator_node;
mod panner_node;
mod wave_shaper_node;

pub use analyser_node::*;
pub use audio_worklet_node::*;
pub use biquad_filter_node::*;
pub use convolver_node::*;
pub use delay_node::*;
pub use dynamics_compressor_node::*;
pub use gain_node::*;
pub use media_stream_destination_node::*;
pub use oscillator_node::*;
pub use panner_node::*;
pub use wave_shaper_node::*;

pub enum AudioNode {
    AnalyserNode(AnalyserNode),
    AudioWorkletNode(AudioWorkletNode),
    BiquadFilterNode(BiquadFilterNode),
    ConvolverNode(ConvolverNode),
    DelayNode(DelayNode),
    DynamicsCompressorNode(DynamicsCompressorNode),
    GainNode(GainNode),
    MediaStreamAudioDestinationNode(MediaStreamAudioDestinationNode),
    OscillatorNode(OscillatorNode),
    PannerNode(PannerNode),
    WaveShaperNode(WaveShaperNode),
}

impl From<AnalyserNode> for AudioNode {
    fn from(node: AnalyserNode) -> Self {
        AudioNode::AnalyserNode(node)
    }
}

impl From<AudioWorkletNode> for AudioNode {
    fn from(node: AudioWorkletNode) -> Self {
        AudioNode::AudioWorkletNode(node)
    }
}

impl From<BiquadFilterNode> for AudioNode {
    fn from(node: BiquadFilterNode) -> Self {
        AudioNode::BiquadFilterNode(node)
    }
}

impl From<ConvolverNode> for AudioNode {
    fn from(node: ConvolverNode) -> Self {
        AudioNode::ConvolverNode(node)
    }
}

impl From<DelayNode> for AudioNode {
    fn from(node: DelayNode) -> Self {
        AudioNode::DelayNode(node)
    }
}

impl From<DynamicsCompressorNode> for AudioNode {
    fn from(node: DynamicsCompressorNode) -> Self {
        AudioNode::DynamicsCompressorNode(node)
    }
}

impl From<GainNode> for AudioNode {
    fn from(node: GainNode) -> Self {
        AudioNode::GainNode(node)
    }
}

impl From<MediaStreamAudioDestinationNode> for AudioNode {
    fn from(node: MediaStreamAudioDestinationNode) -> Self{
        AudioNode::MediaStreamAudioDestinationNode(node)
    }
}

impl From<OscillatorNode> for AudioNode {
    fn from(node: OscillatorNode) -> Self {
        AudioNode::OscillatorNode(node)
    }
}

impl From<PannerNode> for AudioNode {
    fn from(node: PannerNode) -> Self {
        AudioNode::PannerNode(node)
    }
}

impl From<WaveShaperNode> for AudioNode {
    fn from(node: WaveShaperNode) -> Self {
        AudioNode::WaveShaperNode(node)
    }
}
