use super::Task;

pub trait Batch {
    fn poll(&mut self) -> Option<Task>;
}
