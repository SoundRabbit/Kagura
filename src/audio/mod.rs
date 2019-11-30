mod node;

use std::collections::HashMap;
use std::collections::HashSet;

pub enum AudioNode {
    Oscillator(node::Oscillator),
    Destination(node::Destination),
}

pub struct AudioNodeConnection {
    audio_nodes: Vec<AudioNode>,
    connection: HashMap<usize, HashSet<usize>>,
}

impl AudioNodeConnection {
    pub fn new() -> Self {
        AudioNodeConnection {
            audio_nodes: vec![],
            connection: HashMap::new(),
        }
    }

    pub fn audio_node(&self, index: usize) -> Option<&AudioNode> {
        self.audio_nodes.get(index)
    }

    pub fn connection(&self, from: &usize, to: &usize) -> bool {
        if let Some(a) = self.connection.get(&from) {
            a.get(&to).is_some()
        } else {
            false
        }
    }

    pub fn append(&mut self, audio_node: AudioNode) -> usize {
        self.audio_nodes.push(audio_node);
        self.audio_nodes.len()
    }

    pub fn connect(&mut self, from: usize, to: usize) {
        if let Some(a) = self.connection.get_mut(&from) {
            a.insert(to);
        } else {
            let mut a: HashSet<usize> = HashSet::new();
            a.insert(to);
            self.connection.insert(from, a);
        }
    }
}
