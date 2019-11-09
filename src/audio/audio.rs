use super::Connection;
use super::Node;

fn node(audio_node: impl Into<Node>) -> Connection {
    Connection::Node(audio_node.into())
}

fn pipeline(connection: Vec<Connection>) -> Connection {
    Connection::Pipeline(connection)
}

fn branch(
    spliter: impl Into<super::Spliter>,
    connection: Vec<Connection>,
    merger: impl Into<super::Merger>,
) -> Connection {
    Connection::Branch(spliter.into(), connection, merger.into())
}
