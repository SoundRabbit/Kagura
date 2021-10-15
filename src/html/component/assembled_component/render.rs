use super::*;
use crate::kagura::node;
use crate::libs::diff_mix;

impl<ThisComp: Update + Render, DemirootComp: Component>
    AssembledComponentInstance<ThisComp, DemirootComp>
{
    pub fn render(&mut self, html_children: Vec<Html<DemirootComp>>) -> VecDeque<Node> {
        let html_children = html_children
            .into_iter()
            .map(|child| self.wrap(child))
            .collect();

        let mut children = vec![];
        std::mem::swap(&mut self.children, &mut children);

        let (children, html_children) = self.update_children(children.into(), html_children);
        self.children = children.into();
        let html = self.data.borrow().render(&self.props, html_children.into());

        let mut before = ComponentTree::None;
        std::mem::swap(&mut self.children_tree, &mut before);

        let (after, nodes) = self.render_html(before, html);

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
                ref_marker,
            } => Html::ElementNode {
                tag_name,
                children: children.into_iter().map(|node| self.wrap(node)).collect(),
                attributes,
                events: Self::wrap_events::<ThisComp, DemirootComp>(events, &self.demiroot),
                ref_marker: Self::wrap_ref_marker::<ThisComp, DemirootComp>(
                    ref_marker,
                    &self.demiroot,
                ),
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
                                crate::state::render();
                            }
                        })))
                    }
                }
            }

            wrapped.handler_table.insert(event_name, wrapped_handlers);
        }

        wrapped
    }

    fn wrap_ref_marker<T: Component, D: Component>(
        ref_markers: Vec<RefMarker<D>>,
        demiroot: &Option<Weak<RefCell<dyn AssembledDemirootComponent<ThisComp = D>>>>,
    ) -> Vec<RefMarker<T>> {
        let mut wrapped = vec![];

        for ref_marker in ref_markers {
            match ref_marker {
                RefMarker::WrappedRef(marker) => {
                    wrapped.push(RefMarker::WrappedRef(marker));
                }
                RefMarker::RefString(marker) => {
                    let demiroot = demiroot.as_ref().map(|x| Weak::clone(x));
                    wrapped.push(RefMarker::WrappedRef(Box::new(move |node| {
                        if let Some(demiroot) = demiroot.as_ref().and_then(Weak::upgrade) {
                            demiroot.borrow_mut().ref_node(marker.name, node);
                        }
                    })));
                }
            }
        }

        wrapped
    }

    fn update_children(
        &self,
        befores: VecDeque<ComponentTree<ThisComp, DemirootComp>>,
        afters: VecDeque<Html<ThisComp>>,
    ) -> (
        VecDeque<ComponentTree<ThisComp, DemirootComp>>,
        VecDeque<Html<ThisComp>>,
    ) {
        let mut mixeds = diff_mix(befores, afters, Self::compare_component_html, 1.0, 1.0, 1.0);
        let mut res_c = VecDeque::new();
        let mut res_h = VecDeque::new();

        while let Some((before, after)) = mixeds.pop_front() {
            match after {
                None => {}
                Some(after) => match after {
                    Html::TextNode { text, events } => {
                        res_c.push_back(ComponentTree::TextNode);
                        res_h.push_back(Html::TextNode { text, events });
                    }
                    Html::ElementNode {
                        tag_name,
                        children,
                        attributes,
                        events,
                        ref_marker,
                    } => match before {
                        Some(ComponentTree::Element(befores)) => {
                            let (children_c, children_h) =
                                self.update_children(befores, children.into());
                            res_c.push_back(ComponentTree::Element(children_c));
                            res_h.push_back(Html::ElementNode {
                                tag_name,
                                children: children_h.into(),
                                attributes,
                                events,
                                ref_marker,
                            });
                        }
                        _ => {
                            let (children_c, children_h) =
                                self.update_children(VecDeque::new(), children.into());
                            res_c.push_back(ComponentTree::Element(children_c));
                            res_h.push_back(Html::ElementNode {
                                tag_name,
                                children: children_h.into(),
                                attributes,
                                events,
                                ref_marker,
                            });
                        }
                    },

                    Html::Fragment(afters) => match before {
                        Some(ComponentTree::Fragment(befores)) => {
                            let (afters_c, afters_h) = self.update_children(befores, afters.into());
                            res_c.push_back(ComponentTree::Fragment(afters_c));
                            res_h.push_back(Html::Fragment(afters_h.into()));
                        }
                        _ => {
                            let (afters_c, afters_h) =
                                self.update_children(VecDeque::new(), afters.into());
                            res_c.push_back(ComponentTree::Fragment(afters_c));
                            res_h.push_back(Html::Fragment(afters_h.into()));
                        }
                    },

                    Html::ComponentNode(ComponentNode::PackedComponentNode(mut packed)) => {
                        let assembled = match before {
                            Some(ComponentTree::ThisComp(before)) => packed.assemble(Some(before)),
                            _ => packed.assemble(None),
                        };

                        let (assembled, rendered) =
                            Self::render_assembled(assembled, self.this_as_demiroot());

                        res_c.push_back(ComponentTree::ThisComp(Rc::clone(&assembled)));
                        res_h.push_back(Html::ComponentNode(
                            ComponentNode::AssembledComponentNode(
                                AssembledComponentNode::rendered(assembled, rendered),
                            ),
                        ));
                    }

                    Html::ComponentNode(ComponentNode::WrappedPackedComponentNode(wrapped)) => {
                        let mut wrapped = wrapped
                            .downcast::<WrappedPackedComponentNode<DemirootComp>>()
                            .unwrap();
                        let assembled = match before {
                            Some(ComponentTree::DemirootComp(before)) => {
                                wrapped.assemble(Some(before))
                            }
                            _ => wrapped.assemble(None),
                        };
                        let (assembled, rendered) =
                            Self::render_assembled(assembled, self.demiroot_clone());
                        let assembled_node =
                            AssembledComponentNode::rendered(Rc::clone(&assembled), rendered);

                        res_c.push_back(ComponentTree::DemirootComp(assembled));
                        res_h.push_back(Html::ComponentNode(
                            ComponentNode::WrappedAssembledComponentNode(assembled_node.wrap()),
                        ));
                    }

                    Html::ComponentNode(ComponentNode::AssembledComponentNode(assembled)) => {
                        let (assembled, rendered) =
                            Self::render_assembled(assembled, self.this_as_demiroot());

                        res_c.push_back(ComponentTree::ThisComp(Rc::clone(&assembled)));
                        res_h.push_back(Html::ComponentNode(
                            ComponentNode::AssembledComponentNode(
                                AssembledComponentNode::rendered(assembled, rendered),
                            ),
                        ));
                    }

                    Html::ComponentNode(ComponentNode::WrappedAssembledComponentNode(wrapped)) => {
                        let assembled = wrapped
                            .downcast::<WrappedAssembledComponentNode<DemirootComp>>()
                            .unwrap()
                            .take();
                        let (assembled, rendered) =
                            Self::render_assembled(assembled, self.demiroot_clone());
                        let assembled_node =
                            AssembledComponentNode::rendered(Rc::clone(&assembled), rendered);

                        res_c.push_back(ComponentTree::DemirootComp(assembled));
                        res_h.push_back(Html::ComponentNode(
                            ComponentNode::WrappedAssembledComponentNode(assembled_node.wrap()),
                        ));
                    }
                },
            }
        }

        (res_c, res_h)
    }

    fn compare_component_html(
        x: &ComponentTree<ThisComp, DemirootComp>,
        y: &Html<ThisComp>,
    ) -> bool {
        match y {
            Html::ComponentNode(ComponentNode::PackedComponentNode(_))
            | Html::ComponentNode(ComponentNode::AssembledComponentNode(_)) => match x {
                ComponentTree::ThisComp(_) => true,
                _ => false,
            },
            Html::ComponentNode(ComponentNode::WrappedPackedComponentNode(_))
            | Html::ComponentNode(ComponentNode::WrappedAssembledComponentNode(_)) => match x {
                ComponentTree::DemirootComp(_) => true,
                _ => false,
            },
            Html::TextNode { .. } => match x {
                ComponentTree::TextNode => true,
                _ => false,
            },
            Html::ElementNode { .. } => match x {
                ComponentTree::Element(_) => true,
                _ => false,
            },
            Html::Fragment(_) => match x {
                ComponentTree::Fragment(_) => true,
                _ => false,
            },
        }
    }

    fn render_html(
        &mut self,
        before: ComponentTree<ThisComp, DemirootComp>,
        after: Html<ThisComp>,
    ) -> (ComponentTree<ThisComp, DemirootComp>, VecDeque<Node>) {
        let mut before = before.into_deq();
        let afters = Self::flatten_html(after);
        let mut mapped = VecDeque::new();
        let mut nodes = VecDeque::new();

        for after in afters {
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
                    let msgs = assembled.borrow_mut().load_lazy_cmd();
                    for msg in msgs {
                        self.lazy_update(msg);
                    }
                    (ComponentTree::ThisComp(assembled), nodes)
                }
                Html::ComponentNode(ComponentNode::WrappedPackedComponentNode(wrapped)) => {
                    let mut wrapped = wrapped
                        .downcast::<WrappedPackedComponentNode<DemirootComp>>()
                        .unwrap();
                    let assembled = match before_child {
                        ComponentTree::DemirootComp(before_child) => {
                            wrapped.assemble(Some(before_child))
                        }
                        _ => wrapped.assemble(None),
                    };
                    let (assembled, nodes) =
                        Self::render_assembled(assembled, self.demiroot_clone());
                    let msgs = assembled.borrow_mut().load_lazy_cmd();
                    for msg in msgs {
                        self.lazy_cmd.push_back(AssembledCmd::Msg(msg));
                    }
                    (ComponentTree::DemirootComp(assembled), nodes)
                }
                Html::ComponentNode(ComponentNode::AssembledComponentNode(assembled)) => {
                    let (assembled, nodes) =
                        Self::render_assembled(assembled, self.this_as_demiroot());
                    let msgs = assembled.borrow_mut().load_lazy_cmd();
                    for msg in msgs {
                        self.lazy_update(msg);
                    }
                    (ComponentTree::ThisComp(assembled), nodes)
                }
                Html::ComponentNode(ComponentNode::WrappedAssembledComponentNode(wrapped)) => {
                    let assembled = wrapped
                        .downcast::<WrappedAssembledComponentNode<DemirootComp>>()
                        .unwrap()
                        .take();
                    let (assembled, nodes) =
                        Self::render_assembled(assembled, self.demiroot_clone());
                    let msgs = assembled.borrow_mut().load_lazy_cmd();
                    for msg in msgs {
                        self.lazy_cmd.push_back(AssembledCmd::Msg(msg));
                    }
                    (ComponentTree::DemirootComp(assembled), nodes)
                }
                Html::TextNode { text, events } => (
                    ComponentTree::None,
                    vec![Node::text(text, self.get_node_events(events))].into(),
                ),
                Html::ElementNode {
                    tag_name,
                    children,
                    attributes,
                    events,
                    ref_marker,
                } => {
                    let mut before_children = match before_child {
                        ComponentTree::Fragment(before_children) => before_children,
                        _ => std::collections::VecDeque::new(),
                    };
                    let mut fragment = std::collections::VecDeque::new();
                    let mut nodes = VecDeque::new();
                    for child in children {
                        let before_child =
                            before_children.pop_front().unwrap_or(ComponentTree::None);
                        let (c, mut n) = self.render_html(before_child, child);
                        fragment.push_back(c);
                        nodes.append(&mut n);
                    }

                    let ref_marker = Self::wrap_ref_marker::<ThisComp, ThisComp>(
                        ref_marker,
                        &self.this_as_demiroot(),
                    );
                    let ref_marker = ref_marker
                        .into_iter()
                        .filter_map(|marker| match marker {
                            RefMarker::WrappedRef(marker) => Some(marker),
                            _ => None,
                        })
                        .collect();

                    let node = Node::element(
                        tag_name,
                        attributes.attributes,
                        self.get_node_events(events),
                        nodes,
                        ref_marker,
                    );
                    (ComponentTree::Fragment(fragment), vec![node].into())
                }
                Html::Fragment(..) => {
                    unreachable!();
                }
            };
            mapped.push_back(m);
            nodes.append(&mut n);
        }

        if mapped.len() == 0 {
            (ComponentTree::None, vec![].into())
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
        VecDeque<Node>,
    ) {
        match assembled.payload {
            AssembledComponentNodePayload::Children(children) => {
                let assembled = assembled.data;
                assembled.borrow_mut().set_demiroot(demiroot);
                let nodes = assembled.borrow_mut().render(children);
                (assembled, nodes)
            }
            AssembledComponentNodePayload::Rendered(rendered) => (assembled.data, rendered),
        }
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
