use crate::html::component::assembled_component::AssembledComponentInstance;
use crate::html::component::Sub;
use crate::Html;
use std::cell::RefCell;
use std::rc::Rc;

mod document;
pub mod node;
mod renderer;
mod terminator;

use document::Document;
pub use node::Node;
use renderer::Renderer;
pub use terminator::Terminator;

pub struct Kagura {
    instance: Rc<RefCell<AssembledComponentInstance<Document, Terminator>>>,
    children: Box<dyn FnMut() -> Vec<Html<Terminator>>>,
    r_root: web_sys::Node,
    renderer: Renderer,
}

impl Kagura {
    pub fn mount(
        root_node: web_sys::Node,
        children: impl FnMut() -> Vec<Html<Terminator>> + 'static,
    ) {
        let document = Document::new();
        let instance = AssembledComponentInstance::new_ref(
            Rc::new(RefCell::new(document)),
            document::Props {},
            Sub::none(),
        );
        let this = Self {
            instance,
            children: Box::new(children),
            r_root: root_node,
            renderer: Renderer::new(),
        };
        crate::state::mount(this);
    }

    pub fn render(&mut self) {
        let children = (self.children)();
        let afters = self.instance.borrow_mut().render(children);

        self.renderer.render(afters, &self.r_root);
    }
}
