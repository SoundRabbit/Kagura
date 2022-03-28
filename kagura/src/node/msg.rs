use crate::Component;
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

    pub fn target_id<Target: Component>(target: &Target) -> usize {
        target as *const Target as usize
    }

    pub fn target_is<Target: Component>(&self, c: &Target) -> bool {
        self.target == c as *const Target as usize
    }

    pub fn take(&mut self) -> Option<Box<dyn Any>> {
        self.data.borrow_mut().take()
    }
}
