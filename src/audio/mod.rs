mod node;

pub enum Node {
    Oscillator(node::Oscillator),
    Destination(node::Destination),
}
