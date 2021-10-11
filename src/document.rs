use std::cell::RefCell;
use std::rc::Rc;

pub mod env;
pub mod node;

pub use env::Env;
pub use node::Node;

use crate::html::component::{Constructor, Render, Update};
use crate::html::Component;
use crate::Html;

pub struct Props {
    root: web_sys::Node,
}

pub enum Msg {
    None,
}

pub enum On {}

pub struct Document {
    r_document: web_sys::Document,
    r_root: web_sys::Node,
    env: Rc<RefCell<Env>>,
    before_node: Vec<Node>,
}

impl Document {}

impl Component for Document {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for Document {
    fn constructor(props: &Props) -> Self {
        let r_document = web_sys::window().unwrap().document().unwrap();
        let r_root = props.root.clone();
        let env = Env::new_ref();
        let before_node = vec![];
        Self {
            r_document,
            r_root,
            env,
            before_node,
        }
    }
}

impl Update for Document {}

impl Render for Document {
    fn render(&self, props: &Props, children: Vec<Html<Self>>) -> Html<Self> {
        // Documentは何も返さない
        Html::none()
    }
}
