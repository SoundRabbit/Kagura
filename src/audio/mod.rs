mod node;
mod renderer;

pub enum Node {
    Oscillator(node::Oscillator),
    Destination(node::Destination),
}
