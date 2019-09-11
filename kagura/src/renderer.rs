use crate::dom;
use crate::native;

pub struct Renderer {
    before: dom::Node,
    root: native::Node,
}

impl Renderer {
    pub fn new(virtual_node: dom::Node, root_node: native::Node) -> Self {
        let before = virtual_node.clone();
        let root = Self::render_all(virtual_node);
        root_node.parent_node().replace_child(&root, &root_node);
        Self { before, root }
    }

    pub fn update(&mut self, after: dom::Node) {
        self.before = after.clone();
        if let Some(root) = Self::render_component(after, &self.root, &self.root.parent_node()) {
            self.root = root;
        }
    }

    fn render_all(virtual_node: dom::Node) -> native::Node {
        match virtual_node {
            dom::Node::Text(text) => native::create_text_node(&text).into(),
            dom::Node::Element {
                tag_name,
                attributes: _,
                events: _,
                children,
                rerender: _,
            } => {
                let root = native::create_element(&tag_name);

                for child in children {
                    let child = Self::render_all(child);
                    root.append_child(&child);
                }

                root.into()
            }
        }
    }

    fn render_component(
        virtual_node: dom::Node,
        root: &native::Node,
        parent: &native::Node,
    ) -> Option<native::Node> {
        match virtual_node {
            dom::Node::Text(_) => (None),
            dom::Node::Element {
                tag_name,
                attributes,
                events,
                children,
                rerender,
            } => {
                if rerender {
                    let new_root = Self::render_all(dom::Node::element(
                        tag_name, attributes, events, children, rerender,
                    ));
                    parent.replace_child(&new_root, root);
                    Some(new_root)
                } else {
                    let mut i: usize = 0;
                    for child in children {
                        if let Some(node) = root.children().item(i) {
                            Self::render_component(child, &node, &root);
                        }
                        i += 1;
                    }
                    None
                }
            }
        }
    }
}
