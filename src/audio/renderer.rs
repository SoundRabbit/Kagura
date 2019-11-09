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
            let mut audio_nodes: Vec<AudioNode> = connections
                .into_iter()
                .map(|connection| render(connection))
                .collect();
            for i in 0..(audio_nodes.len()) {
                if let (Some(prev_nodes), Some(next_nodes)) =
                    (audio_nodes.get(i), audio_nodes.get(i + 1))
                {
                    let (_, prev_nodes) = prev_nodes;
                    let (next_nodes, _) = next_nodes;
                    for prev_node in prev_nodes {
                        for next_node in next_nodes {
                            prev_node.connect_with_audio_node(next_node);
                        }
                    }
                }
            }
            if audio_nodes.len() > 0 {
                let (first_nodes, last_nodes) = (
                    audio_nodes.remove(0),
                    audio_nodes.remove(audio_nodes.len() - 1),
                );
                let (first_nodes, _) = first_nodes;
                let (_, last_nodes) = last_nodes;
                (first_nodes, last_nodes)
            } else {
                (vec![], vec![])
            }
        }
        _ => (vec![], vec![]),
    }
}
