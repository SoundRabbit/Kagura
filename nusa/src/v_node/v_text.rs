use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct VText {
    pub text: Rc<String>,
}
