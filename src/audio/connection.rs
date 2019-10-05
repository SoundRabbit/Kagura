pub use super::node::*;

pub trait Connector<T> {
    fn connect(self, v: T) -> Self;
}

pub enum Connection {
    Nodes(Vec<Connection>),
    Node(AudioNode),
}

impl Connection {
    pub fn new() -> Self {
        Connection::Nodes(vec![])
    }

    pub fn branch(mut self, branches: Vec<Connection>) -> Self {
        if let Connection::Nodes(connections) = &mut self {
            connections.push(Connection::Nodes(branches));
        }
        self
    }
}

impl Connector<AnalyserNode> for Connection {
    fn connect(mut self, node: AnalyserNode) -> Self {
        if let Connection::Nodes(connections) = &mut self {
            connections.push(Connection::Node(AudioNode::from(node)));
        }
        self
    }
}

impl Connector<AudioWorkletNode> for Connection {
    fn connect(mut self, node: AudioWorkletNode) -> Self {
        if let Connection::Nodes(connections) = &mut self {
            connections.push(Connection::Node(AudioNode::from(node)));
        }
        self
    }
}

impl Connector<BiquadFilterNode> for Connection {
    fn connect(mut self, node: BiquadFilterNode) -> Self {
        if let Connection::Nodes(connections) = &mut self {
            connections.push(Connection::Node(AudioNode::from(node)));
        }
        self
    }
}

impl Connector<ConvolverNode> for Connection {
    fn connect(mut self, node: ConvolverNode) -> Self {
        if let Connection::Nodes(connections) = &mut self {
            connections.push(Connection::Node(AudioNode::from(node)));
        }
        self
    }
}

impl Connector<DelayNode> for Connection {
    fn connect(mut self, node: DelayNode) -> Self {
        if let Connection::Nodes(connections) = &mut self {
            connections.push(Connection::Node(AudioNode::from(node)));
        }
        self
    }
}

impl Connector<DynamicsCompressorNode> for Connection {
    fn connect(mut self, node: DynamicsCompressorNode) -> Self {
        if let Connection::Nodes(connections) = &mut self {
            connections.push(Connection::Node(AudioNode::from(node)));
        }
        self
    }
}

impl Connector<GainNode> for Connection {
    fn connect(mut self, node: GainNode) -> Self {
        if let Connection::Nodes(connections) = &mut self {
            connections.push(Connection::Node(AudioNode::from(node)));
        }
        self
    }
}

impl Connector<PannerNode> for Connection {
    fn connect(mut self, node: PannerNode) -> Self {
        if let Connection::Nodes(connections) = &mut self {
            connections.push(Connection::Node(AudioNode::from(node)));
        }
        self
    }
}

impl Connector<WaveShaperNode> for Connection {
    fn connect(mut self, node: WaveShaperNode) -> Self {
        if let Connection::Nodes(connections) = &mut self {
            connections.push(Connection::Node(AudioNode::from(node)));
        }
        self
    }
}
