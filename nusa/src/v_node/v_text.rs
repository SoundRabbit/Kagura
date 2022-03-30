use std::rc::Rc;

#[derive(Clone)]
pub struct VText {
    pub text: Rc<String>,
}
