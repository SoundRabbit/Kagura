use std::any::Any;
use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;

#[derive(Clone)]
pub struct Msg {
    target: usize,
    data: Rc<RefCell<Option<Box<dyn Any>>>>,
}

pub type FutureMsg = Pin<Box<dyn Future<Output = Msg>>>;

impl Msg {
    pub fn new(target: usize, data: Box<dyn Any>) -> Self {
        Self {
            target,
            data: Rc::new(RefCell::new(Some(data))),
        }
    }

    pub fn target_id<Component>(target: &Component) -> usize {
        target as *const Component as usize
    }

    pub fn target_is<Component>(&self, c: &Component) -> bool {
        self.target == c as *const Component as usize
    }

    pub fn take(&mut self) -> Option<Box<dyn Any>> {
        self.data.borrow_mut().take()
    }
}
