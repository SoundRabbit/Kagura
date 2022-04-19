use std::rc::Rc;

#[derive(Clone)]
pub struct VText {
    pub text: Rc<String>,
}

impl std::fmt::Debug for VText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>", self.text.as_str())
    }
}
