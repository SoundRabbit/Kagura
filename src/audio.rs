pub mod connection;
pub mod node;

pub enum Connection {
    Nodes(Vec<Connection>),
    Node(AudioNode),
}

pub struct AudioNode {
    pub node: node::AudioNode,
    pub id: u128,
}