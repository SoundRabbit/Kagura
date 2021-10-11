use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

pub type IdType = u64;

pub struct Env {
    count: IdType,
    handlers: HashMap<IdType, Box<dyn FnOnce(web_sys::Event)>>,
    tasks: VecDeque<Box<dyn FnOnce()>>,
}

impl Env {
    pub fn new_ref() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            count: 0,
            handlers: HashMap::new(),
            tasks: VecDeque::new(),
        }))
    }

    pub fn gen_id(&mut self) -> IdType {
        let id = self.count;
        self.count += 1;
        id
    }

    pub fn add_handler(
        &mut self,
        handler_id: IdType,
        handler: impl FnOnce(web_sys::Event) + 'static,
    ) -> IdType {
        self.handlers.insert(handler_id, Box::new(handler));
        handler_id
    }

    pub fn take_handler(&mut self, handler_id: &IdType) -> Option<Box<dyn FnOnce(web_sys::Event)>> {
        self.handlers.remove(handler_id)
    }

    pub fn push_task(&mut self, task: impl FnOnce() + 'static) {
        self.tasks.push_back(Box::new(task));
    }

    pub fn pop_task(&mut self) -> Option<Box<dyn FnOnce()>> {
        self.tasks.pop_front()
    }
}
