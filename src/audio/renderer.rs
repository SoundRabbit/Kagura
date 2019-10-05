use super::connection::Connection;
use super::context::*;
use super::node::*;
use std::collections::HashMap;

struct AudioRenderer {
    contexts: HashMap<u128, AudioContext>,
}

impl AudioRenderer {
    fn new(connections: Vec<(Connection, u128)>) -> Self {
        let mut contexts: HashMap<u128, AudioContext> = HashMap::new();
        for (connection, component_id) in connections {
            let mut context = AudioContext::new();
            context.set_connection(connection);
            let context = context.render();
            contexts.insert(component_id, context);
        }
        Self { contexts }
    }
}

impl AudioContext {
    fn render(mut self) -> Self {
        let mut after_nodes: HashMap<u128, web_sys::AudioNode> = HashMap::new();
        AudioContext::render_diff(
            &self.context,
            &mut after_nodes,
            &mut self.nodes,
            &self.connection,
            &self.last_connection,
            None,
            RenderMode::Serial
        );
        self.last_connection = self.connection;
        self.connection = ConnectionContext::None;
        self.nodes = after_nodes;
        self
    }

    fn render_diff<'a> (
        context: &web_sys::AudioContext,
        after_nodes: &'a mut HashMap<u128, web_sys::AudioNode>,
        before_nodes: &mut HashMap<u128, web_sys::AudioNode>,
        after: &ConnectionContext,
        before: &ConnectionContext,
        prev: Option<Vec<&web_sys::AudioNode>>,
        mode: RenderMode
    ) -> Option<&'a web_sys::AudioNode> {
        match after {
            ConnectionContext::Node(node_context) => {
                match &node_context.node {
                    AudioNode::AnalyserNode(props) => {
                        let node = context.create_analyser().unwrap();
                        after_nodes.insert(node_context.id, node.into());
                        after_nodes.get(&node_context.id)
                    }
                    AudioNode::AudioWorkletNode(props) => {
                        let node = context.create_analyser().unwrap();
                        after_nodes.insert(node_context.id, node.into());
                        after_nodes.get(&node_context.id)
                    }
                    AudioNode::BiquadFilterNode(props) => {
                        let node = context.create_biquad_filter().unwrap();
                        after_nodes.insert(node_context.id, node.into());
                        after_nodes.get(&node_context.id)
                    }
                    AudioNode::ConvolverNode(props) => {
                        let node = context.create_convolver().unwrap();
                        after_nodes.insert(node_context.id, node.into());
                        after_nodes.get(&node_context.id)
                    }
                    AudioNode::DelayNode(props) => {
                        let node = context.create_delay().unwrap();
                        after_nodes.insert(node_context.id, node.into());
                        after_nodes.get(&node_context.id)
                    }
                    AudioNode::DynamicsCompressorNode(props) => {
                        let node = context.create_dynamics_compressor().unwrap();
                        after_nodes.insert(node_context.id, node.into());
                        after_nodes.get(&node_context.id)
                    }
                    AudioNode::GainNode(props) => {
                        let node = context.create_gain().unwrap();
                        after_nodes.insert(node_context.id, node.into());
                        after_nodes.get(&node_context.id)
                    }
                    AudioNode::PannerNode(props) => {
                        let node = context.create_panner().unwrap();
                        after_nodes.insert(node_context.id, node.into());
                        after_nodes.get(&node_context.id)
                    }
                    AudioNode::WaveShaperNode(props) => {
                        let node = context.create_wave_shaper().unwrap();
                        after_nodes.insert(node_context.id, node.into());
                        after_nodes.get(&node_context.id)
                    }
                }
            }
            ConnectionContext::Nodes(nodes) => {
                match mode {
                    RenderMode::Parallel => {
                        None
                    }
                    RenderMode::Serial => {
                        None
                    }
                }
            }
            ConnectionContext::None => None
        }
    }
}

enum RenderMode {
    Serial,
    Parallel
}

impl RenderMode {
    fn inv(&self) -> Self {
        match self {
            RenderMode::Serial => RenderMode::Parallel,
            RenderMode::Parallel => RenderMode::Serial
        }
    }
}