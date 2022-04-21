pub mod batch;
pub mod task;

pub use batch::Batch;
pub use task::Task;

pub enum FutureMsg {
    Task(Task),
    Batch(Box<dyn Batch>),
}
