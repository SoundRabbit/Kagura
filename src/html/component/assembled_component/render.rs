use std::any::Any;

use super::*;
use crate::document::node;

impl<ThisComp: Update + Render, DemirootComp: Component>
    AssembledComponentInstance<ThisComp, DemirootComp>
{
    pub fn render(&mut self, children: Vec<Html<DemirootComp>>) -> Vec<Node> {
        let children = children.into_iter().map(|child| self.wrap(child)).collect();
        let html = self.data.borrow().render(&self.props, children);
        let mut before = ComponentTree::None;
        let mut nodes = vec![];

        std::mem::swap(&mut self.children_tree, &mut before);
        self.children.clear();

        let (after, mut n) = self.render_html(before, html);
        nodes.append(&mut n);

        self.children_tree = after;

        nodes
    }

    fn wrap(&self, html: Html<DemirootComp>) -> Html<ThisComp> {
        match html {
            Html::ComponentNode(ComponentNode::PackedComponentNode(mut packed)) => {
                Html::ComponentNode(ComponentNode::WrappedPackedComponentNode(packed.wrap()))
            }
            Html::ComponentNode(ComponentNode::WrappedPackedComponentNode(wrapped)) => {
                Html::ComponentNode(ComponentNode::WrappedPackedComponentNode(wrapped))
            }
            Html::ComponentNode(ComponentNode::AssembledComponentNode(assembled)) => {
                Html::ComponentNode(ComponentNode::WrappedAssembledComponentNode(
                    assembled.wrap(),
                ))
            }
            Html::ComponentNode(ComponentNode::WrappedAssembledComponentNode(wrapped)) => {
                Html::ComponentNode(ComponentNode::WrappedAssembledComponentNode(wrapped))
            }
            Html::TextNode { text, events } => Html::TextNode {
                text,
                events: Self::wrap_events::<ThisComp, DemirootComp>(events, &self.demiroot),
            },
            Html::ElementNode {
                tag_name,
                children,
                attributes,
                events,
            } => Html::ElementNode {
                tag_name,
                children: children.into_iter().map(|node| self.wrap(node)).collect(),
                attributes,
                events: Self::wrap_events::<ThisComp, DemirootComp>(events, &self.demiroot),
            },
            Html::Fragment(nodes) => {
                Html::Fragment(nodes.into_iter().map(|node| self.wrap(node)).collect())
            }
        }
    }

    fn wrap_events<T: Component, D: Component>(
        events: Events<D::Msg>,
        demiroot: &Option<Weak<RefCell<dyn AssembledDemirootComponent<ThisComp = D>>>>,
    ) -> Events<T::Msg> {
        let mut wrapped = Events::new();

        for (event_name, handlers) in events.handler_table {
            let mut wrapped_handlers = vec![];

            for handler in handlers {
                match handler {
                    EventHandler::Wrapped(handler) => {
                        wrapped_handlers.push(EventHandler::Wrapped(handler));
                    }
                    EventHandler::Unrwapped(handler) => {
                        let demiroot = demiroot.as_ref().map(|x| Weak::clone(x));
                        wrapped_handlers.push(EventHandler::Wrapped(Box::new(move |e| {
                            if let Some(demiroot) = demiroot.as_ref().and_then(Weak::upgrade) {
                                let msg = handler(e);
                                demiroot.borrow_mut().update(msg);
                            }
                        })))
                    }
                }
            }

            wrapped.handler_table.insert(event_name, wrapped_handlers);
        }

        wrapped
    }

    fn render_html(
        &mut self,
        before: ComponentTree<ThisComp, DemirootComp>,
        after: Html<ThisComp>,
    ) -> (ComponentTree<ThisComp, DemirootComp>, Vec<Node>) {
        let mut before = before.into_deq();
        let after = Self::flatten_html(after);
        let mut mapped = VecDeque::new();
        let mut nodes = vec![];

        for after in after {
            let before_child = before.pop_front().unwrap_or(ComponentTree::None);

            let (m, mut n) = match after {
                Html::ComponentNode(ComponentNode::PackedComponentNode(mut packed)) => {
                    let assembled = match before_child {
                        ComponentTree::ThisComp(before_child) => {
                            packed.assemble(Some(before_child))
                        }
                        _ => packed.assemble(None),
                    };
                    let (assembled, nodes) =
                        Self::render_assembled(assembled, self.this_as_demiroot());
                    while let Some(msg) = assembled.borrow_mut().load_lazy_cmd() {
                        self.lazy_update(msg);
                    }
                    self.children
                        .push(ChildComponent::ThisComp(Rc::clone(&assembled)));
                    (ComponentTree::ThisComp(assembled), nodes)
                }
                Html::ComponentNode(ComponentNode::WrappedPackedComponentNode(mut wrapped)) => {
                    let wrapped = (&mut wrapped as &mut dyn Any)
                        .downcast_mut::<WrappedPackedComponentNodeInstance<DemirootComp>>()
                        .unwrap();
                    let assembled = match before_child {
                        ComponentTree::DemirootComp(before_child) => {
                            wrapped.assemble(Some(before_child))
                        }
                        _ => wrapped.assemble(None),
                    };
                    let (assembled, nodes) =
                        Self::render_assembled(assembled, self.demiroot_clone());
                    while let Some(msg) = assembled.borrow_mut().load_lazy_cmd() {
                        self.lazy_cmd.push_back(AssembledCmd::Msg(msg));
                    }
                    self.children
                        .push(ChildComponent::DemirootComp(Rc::clone(&assembled)));
                    (ComponentTree::DemirootComp(assembled), nodes)
                }
                Html::ComponentNode(ComponentNode::AssembledComponentNode(assembled)) => {
                    let (assembled, nodes) =
                        Self::render_assembled(assembled, self.this_as_demiroot());
                    self.children
                        .push(ChildComponent::ThisComp(Rc::clone(&assembled)));
                    while let Some(msg) = assembled.borrow_mut().load_lazy_cmd() {
                        self.lazy_update(msg);
                    }
                    (ComponentTree::ThisComp(assembled), nodes)
                }
                Html::ComponentNode(ComponentNode::WrappedAssembledComponentNode(mut wrapped)) => {
                    let assembled = (&mut wrapped as &mut dyn Any)
                        .downcast_mut::<WrappedAssembledComponentNodeInstance<DemirootComp>>()
                        .unwrap()
                        .take();
                    let (assembled, nodes) =
                        Self::render_assembled(assembled, self.demiroot_clone());
                    while let Some(msg) = assembled.borrow_mut().load_lazy_cmd() {
                        self.lazy_cmd.push_back(AssembledCmd::Msg(msg));
                    }
                    self.children
                        .push(ChildComponent::DemirootComp(Rc::clone(&assembled)));
                    (ComponentTree::DemirootComp(assembled), nodes)
                }
                Html::TextNode { text, events } => (
                    ComponentTree::None,
                    vec![Node::text(text, self.get_node_events(events))],
                ),
                Html::ElementNode {
                    tag_name,
                    children,
                    attributes,
                    events,
                } => {
                    let mut before_children = match before_child {
                        ComponentTree::Fragment(before_children) => before_children,
                        _ => std::collections::VecDeque::new(),
                    };
                    let mut fragment = std::collections::VecDeque::new();
                    let mut nodes = vec![];
                    for child in children {
                        let before_child =
                            before_children.pop_front().unwrap_or(ComponentTree::None);
                        let (c, mut n) = self.render_html(before_child, child);
                        fragment.push_back(c);
                        nodes.append(&mut n);
                    }

                    let node = Node::element(
                        tag_name,
                        attributes.attributes,
                        self.get_node_events(events),
                        nodes,
                    );
                    (ComponentTree::Fragment(fragment), vec![node])
                }
                Html::Fragment(..) => {
                    unreachable!();
                }
            };
            mapped.push_back(m);
            nodes.append(&mut n);
        }

        if mapped.len() == 0 {
            (ComponentTree::None, nodes)
        } else if mapped.len() == 1 {
            (mapped.remove(0).unwrap(), nodes)
        } else {
            (ComponentTree::Fragment(mapped), nodes)
        }
    }

    fn render_assembled<C: Component>(
        assembled: AssembledComponentNode<C>,
        demiroot: Option<Weak<RefCell<dyn AssembledDemirootComponent<ThisComp = C>>>>,
    ) -> (
        Rc<RefCell<dyn AssembledChildComponent<DemirootComp = C>>>,
        Vec<Node>,
    ) {
        let children = assembled.children;
        let assembled = assembled.data;

        assembled.borrow_mut().set_demiroot(demiroot);

        let nodes = assembled.borrow_mut().render(children);

        (assembled, nodes)
    }

    fn get_node_events(&self, events: Events<ThisComp::Msg>) -> node::Events {
        let html_events = Self::wrap_events::<ThisComp, ThisComp>(events, &self.this_as_demiroot());
        let mut events = node::Events::new();

        for (event_name, handlers) in html_events.handler_table {
            for handler in handlers {
                if let EventHandler::Wrapped(handler) = handler {
                    events.add(&event_name, handler);
                }
            }
        }

        events
    }

    fn flatten_html<C: Component>(html: Html<C>) -> Vec<Html<C>> {
        match html {
            Html::Fragment(htmls) => {
                let mut flatten_htmls = vec![];
                for html in htmls {
                    flatten_htmls.append(&mut Self::flatten_html(html));
                }
                flatten_htmls
            }
            _ => vec![html],
        }
    }
}
