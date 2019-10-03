pub use super::AnalyzerNode;
use super::AudioNodeType;
pub use super::AudioWorkletNode;
pub use super::BiquadFilterNode;
pub use super::BiquadFilterType;
pub use super::ConvolverNode;
pub use super::DelayNode;
pub use super::DistanceModelType;
pub use super::DynamicsCompressorNode;
pub use super::GainNode;
pub use super::OverSampleType;
pub use super::PannerNode;
pub use super::PanningModelType;
pub use super::WaveShaperNode;

pub trait Connector<T> {
    fn connect(self, v: T) -> Self;
}

pub enum Connection {
    Collection(Vec<Connection>),
    Node(AudioNodeType),
}

impl Connection {
    pub fn new() -> Self {
        Connection::Collection(vec![])
    }
}

impl Connector<AnalyzerNode> for Connection {
    fn connect(mut self, node: AnalyzerNode) -> Self {
        if let Connection::Collection(connections) = &mut self {
            connections.push(Connection::Node(AudioNodeType::Analyzer(node)));
        }
        self
    }
}

impl Connector<AudioWorkletNode> for Connection {
    fn connect(mut self, node: AudioWorkletNode) -> Self {
        if let Connection::Collection(connections) = &mut self {
            connections.push(Connection::Node(AudioNodeType::AudioWorklet(node)));
        }
        self
    }
}

impl Connector<BiquadFilterNode> for Connection {
    fn connect(mut self, node: BiquadFilterNode) -> Self {
        if let Connection::Collection(connections) = &mut self {
            connections.push(Connection::Node(AudioNodeType::BiquadFilter(node)));
        }
        self
    }
}

impl Connector<ConvolverNode> for Connection {
    fn connect(mut self, node: ConvolverNode) -> Self {
        if let Connection::Collection(connections) = &mut self {
            connections.push(Connection::Node(AudioNodeType::Convolver(node)));
        }
        self
    }
}

impl Connector<DelayNode> for Connection {
    fn connect(mut self, node: DelayNode) -> Self {
        if let Connection::Collection(connections) = &mut self {
            connections.push(Connection::Node(AudioNodeType::Delay(node)));
        }
        self
    }
}

impl Connector<DynamicsCompressorNode> for Connection {
    fn connect(mut self, node: DynamicsCompressorNode) -> Self {
        if let Connection::Collection(connections) = &mut self {
            connections.push(Connection::Node(AudioNodeType::DynamicsCompressor(node)));
        }
        self
    }
}

impl Connector<GainNode> for Connection {
    fn connect(mut self, node: GainNode) -> Self {
        if let Connection::Collection(connections) = &mut self {
            connections.push(Connection::Node(AudioNodeType::Gain(node)));
        }
        self
    }
}

impl Connector<PannerNode> for Connection {
    fn connect(mut self, node: PannerNode) -> Self {
        if let Connection::Collection(connections) = &mut self {
            connections.push(Connection::Node(AudioNodeType::Panner(node)));
        }
        self
    }
}

impl Connector<WaveShaperNode> for Connection {
    fn connect(mut self, node: WaveShaperNode) -> Self {
        if let Connection::Collection(connections) = &mut self {
            connections.push(Connection::Node(AudioNodeType::WaveShaper(node)));
        }
        self
    }
}
