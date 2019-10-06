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
            context.render();
            contexts.insert(component_id, context);
        }
        Self { contexts }
    }
}

impl AudioContext {
    fn render(&self) {
        AudioContext::render_force(&self.context, &self.connection, &None, RenderMode::Serial);
    }

    fn render_force(
        context: &web_sys::AudioContext,
        connection: &ConnectionContext,
        prev: &Option<Vec<web_sys::AudioNode>>,
        mode: RenderMode,
    ) -> Option<Vec<web_sys::AudioNode>> {
        match connection {
            ConnectionContext::Node(node_context) => {
                let node = match &node_context.node {
                    AudioNode::AnalyserNode(props) => context.create_analyser().unwrap().into(),
                    AudioNode::AudioWorkletNode(props) => context.create_analyser().unwrap().into(),
                    AudioNode::BiquadFilterNode(props) => {
                        context.create_biquad_filter().unwrap().into()
                    }
                    AudioNode::ConvolverNode(props) => context.create_convolver().unwrap().into(),
                    AudioNode::DelayNode(props) => context.create_delay().unwrap().into(),
                    AudioNode::DynamicsCompressorNode(props) => {
                        context.create_dynamics_compressor().unwrap().into()
                    }
                    AudioNode::GainNode(props) => context.create_gain().unwrap().into(),
                    AudioNode::MediaStreamAudioDestinationNode(props) => context.create_media_stream_destination().unwrap().into(),
                    AudioNode::OscillatorNode(props) => {
                        let node = context.create_oscillator().unwrap();
                        let _ = node.start();
                        node.into()
                    }
                    AudioNode::PannerNode(props) => context.create_panner().unwrap().into(),
                    AudioNode::WaveShaperNode(props) => {
                        context.create_wave_shaper().unwrap().into()
                    }
                };
                if let Some(prev) = prev {
                    for prev_node in prev {
                        prev_node.connect_with_audio_node(&node);
                    }
                }
                Some(vec![node])
            }
            ConnectionContext::Nodes(connections) => match mode {
                RenderMode::Parallel => {
                    let mut nexts: Vec<web_sys::AudioNode> = vec![];
                    for connection in connections {
                        let next = AudioContext::render_force(context, &connection, &prev, mode.inv());
                        if let Some(mut next) = next {
                            nexts.append(&mut next);
                        }
                    }
                    if nexts.len() > 0 {
                        Some(nexts)
                    } else {
                        None
                    }
                },
                RenderMode::Serial => {
                    let mut prev = prev;
                    let mut next: Option<Vec<web_sys::AudioNode>> = None;
                    for connection in connections {
                        next = AudioContext::render_force(context, &connection, prev, mode.inv());
                        prev = &next;
                    }
                    next
                },
            },
            ConnectionContext::None => None,
        }
    }
}

enum RenderMode {
    Serial,
    Parallel,
}

impl RenderMode {
    fn inv(&self) -> Self {
        match self {
            RenderMode::Serial => RenderMode::Parallel,
            RenderMode::Parallel => RenderMode::Serial,
        }
    }
}
