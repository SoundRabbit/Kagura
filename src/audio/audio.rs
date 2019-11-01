use super::Node;
use super::Connection;

fn node(audio_node: impl Into<Node>) -> Connection {
    Connection::Node(audio_node.into())
}

fn pipeline(connection: Vec<Connection>) -> Connection {
    Connection::Pipeline(connection)
}

fn branch() {}