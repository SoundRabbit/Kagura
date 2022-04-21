use crate::Component;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Msg {
    target: usize,
    data: Rc<RefCell<Option<Box<dyn Any>>>>,
    is_lazy: bool,
}

impl Msg {
    pub fn busy(target: usize, data: Box<dyn Any>) -> Self {
        Self {
            target,
            data: Rc::new(RefCell::new(Some(data))),
            is_lazy: false,
        }
    }

    pub fn lazy(target: usize, data: Box<dyn Any>) -> Self {
        Self {
            target,
            data: Rc::new(RefCell::new(Some(data))),
            is_lazy: true,
        }
    }

    pub fn target_id<Target: Component>(target: &Target) -> usize {
        target as *const Target as usize
    }

    pub fn target_is<Target: Component>(&self, c: &Target) -> bool {
        self.target == c as *const Target as usize
    }

    pub fn target(&self) -> usize {
        self.target
    }

    pub fn take(&mut self) -> Option<Box<dyn Any>> {
        self.data.borrow_mut().take()
    }

    pub fn type_is<T: Any>(&self) -> bool {
        if let Some(data) = self.data.borrow().as_ref() {
            data.downcast_ref::<T>().is_some()
        } else {
            false
        }
    }

    pub fn is_lazy(&self) -> bool {
        self.is_lazy
    }
}
