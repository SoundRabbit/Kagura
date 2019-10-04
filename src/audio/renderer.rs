use std::collections::HashMap;
use super::context::*;
use super::connection::Connection;
use super::node::*;

struct AudioRenderer {
    contexts: HashMap<u128, AudioContext>
}

impl AudioRenderer {
    fn new(connections: Vec<(Connection, u128)>) -> Self {
        let mut contexts: HashMap<u128, AudioContext> = HashMap::new();
        for (connection, component_id) in connections {
            let mut context = AudioContext::new();
            context.set_connection(connection);
            contexts.insert(component_id, context);
        }
        Self {
            contexts
        }
    }
}
