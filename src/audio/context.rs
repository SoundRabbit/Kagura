use super::connection::Connection;
use super::node::AudioNode;
use std::collections::HashMap;

pub struct AudioContext {
    pub context: web_sys::AudioContext,
    pub nodes: HashMap<u128, web_sys::AudioNode>,
    pub last_connection: ConnectionContext,
    pub connection: ConnectionContext,
}

pub enum ConnectionContext {
    Nodes(Vec<ConnectionContext>),
    Node(AudioNodeContext),
    None,
}

pub struct AudioNodeContext {
    node: AudioNode,
    id: u128,
}

impl AudioContext {
    pub fn new() -> Self {
        Self {
            context: web_sys::AudioContext::new().unwrap(),
            nodes: HashMap::new(),
            last_connection: ConnectionContext::None,
            connection: ConnectionContext::None,
        }
    }

    pub fn set_connection(&mut self, connection: Connection) -> &mut Self {
        self.connection = ConnectionContext::from(connection);
        self
    }
}

impl From<Connection> for ConnectionContext {
    fn from(connection: Connection) -> Self {
        match connection {
            Connection::Node(node) => ConnectionContext::Node(AudioNodeContext::from(node)),
            Connection::Nodes(connections) => ConnectionContext::Nodes(
                connections
                    .into_iter()
                    .map(|c| ConnectionContext::from(c))
                    .collect(),
            ),
        }
    }
}

impl From<AudioNode> for AudioNodeContext {
    fn from(audio_node: AudioNode) -> Self {
        Self {
            node: audio_node,
            id: rand::random(),
        }
    }
}
