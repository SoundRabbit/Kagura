use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;

type AudioNode = (Vec<web_sys::AudioNode>, Vec<web_sys::AudioNode>);

pub struct Renderer {}

fn render(after: super::Connection) -> AudioNode {
    use super::Connection;
    match after {
        Connection::Node(_) => (vec![], vec![]),
        Connection::Pipeline(connections) => {
            let audio_nodes: Vec<AudioNode> = connections
                .into_iter()
                .map(|connection| render(connection))
                .collect();
            for i in 0..(audio_nodes.len()) {
                if let (Some(prev_nodes), Some(next_nodes)) =
                    (audio_nodes.get(i), audio_nodes.get(i + 1))
                {
                    for prev_node in prev_nodes {
                        for next_node in next_nodes {
                            prev_node.connect_with_audio_node(nexn_node);
                        }
                    }
                }
            }
        }
        Connection::Branch => (vec![], vec![]),
    }
}
