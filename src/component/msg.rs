use std::any::Any;

pub struct Msg {
    target: usize,
    pub data: Box<dyn Any>,
}

impl Msg {
    pub fn target_is<T>(&self, t: &T) -> bool {
        self.target == t as *const T as usize
    }
}
