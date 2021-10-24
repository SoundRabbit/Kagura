use super::*;
use crate::kagura::node;
use crate::libs::diff_mix;

impl<ThisComp: Update + Render, DemirootComp: Component>
    AssembledComponentInstance<ThisComp, DemirootComp>
{
    pub fn render(&mut self, html_children: Vec<Html<DemirootComp>>) -> VecDeque<Node> {
        let mut children = vec![];
        std::mem::swap(&mut self.children, &mut children);

        let (children, html_children) = self.wrap_children(children.into(), html_children.into());
        self.children = children.into();
        let html = self.data.borrow().render(&self.props, html_children.into());

        let mut before = ComponentTree::None;
        std::mem::swap(&mut self.children_tree, &mut before);

        let (after, nodes) = self.render_html(before, html);

        self.children_tree = after;

        nodes
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

    fn wrap_children(
        &self,
        befores: VecDeque<ComponentTree<ThisComp, DemirootComp>>,
        afters: VecDeque<Html<DemirootComp>>,
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
                        res_h.push_back(Html::TextNode {
                            text,
                            events: Self::wrap_events::<ThisComp, DemirootComp>(
                                events,
                                &self.demiroot,
                            ),
                        });
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
                                self.wrap_children(befores, children.into());
                            res_c.push_back(ComponentTree::Element(children_c));
                            res_h.push_back(Html::ElementNode {
                                tag_name,
                                children: children_h.into(),
                                attributes,
                                events: Self::wrap_events::<ThisComp, DemirootComp>(
                                    events,
                                    &self.demiroot,
                                ),
                                ref_marker: Self::wrap_ref_marker(ref_marker, &self.demiroot),
                            });
                        }
                        _ => {
                            let (children_c, children_h) =
                                self.wrap_children(VecDeque::new(), children.into());
                            res_c.push_back(ComponentTree::Element(children_c));
                            res_h.push_back(Html::ElementNode {
                                tag_name,
                                children: children_h.into(),
                                attributes,
                                events: Self::wrap_events::<ThisComp, DemirootComp>(
                                    events,
                                    &self.demiroot,
                                ),
                                ref_marker: Self::wrap_ref_marker(ref_marker, &self.demiroot),
                            });
                        }
                    },

                    Html::Fragment(afters) => match before {
                        Some(ComponentTree::Fragment(befores)) => {
                            let (afters_c, afters_h) = self.wrap_children(befores, afters.into());
                            res_c.push_back(ComponentTree::Fragment(afters_c));
                            res_h.push_back(Html::Fragment(afters_h.into()));
                        }
                        _ => {
                            let (afters_c, afters_h) =
                                self.wrap_children(VecDeque::new(), afters.into());
                            res_c.push_back(ComponentTree::Fragment(afters_c));
                            res_h.push_back(Html::Fragment(afters_h.into()));
                        }
                    },

                    Html::ComponentNode(ComponentNode::PackedComponentNode(mut packed)) => {
                        let assembled = match before {
                            Some(ComponentTree::DemirootComp(before)) => {
                                packed.assemble(Some(before))
                            }
                            _ => packed.assemble(None),
                        };

                        let (assembled, rendered) =
                            Self::render_assembled(assembled, self.demiroot_clone());

                        res_c.push_back(ComponentTree::DemirootComp(Rc::clone(&assembled)));

                        let wrapped = WrappedAssembledComponentInstance::wrap(assembled);

                        res_h.push_back(Html::ComponentNode(
                            ComponentNode::WrappedAssembledComponentNode(
                                WrappedAssembledComponentNode {
                                    data: wrapped,
                                    rendered: rendered,
                                },
                            ),
                        ));
                    }

                    Html::ComponentNode(ComponentNode::PrepackedComponentNode(mut prepacked)) => {
                        let assembled = match before {
                            Some(ComponentTree::DemirootComp(before)) => {
                                prepacked.assemble(Some(before))
                            }
                            _ => prepacked.assemble(None),
                        };

                        let (assembled, rendered) =
                            Self::render_assembled(assembled, self.demiroot_clone());

                        res_c.push_back(ComponentTree::DemirootComp(Rc::clone(&assembled)));
                        let wrapped = WrappedAssembledComponentInstance::wrap(assembled);

                        res_h.push_back(Html::ComponentNode(
                            ComponentNode::WrappedAssembledComponentNode(
                                WrappedAssembledComponentNode {
                                    data: wrapped,
                                    rendered: rendered,
                                },
                            ),
                        ));
                    }

                    Html::ComponentNode(ComponentNode::WrappedAssembledComponentNode(wrapped)) => {
                        res_c.push_back(ComponentTree::DemirootComp(Rc::clone(&wrapped.data)));
                        res_h.push_back(Html::ComponentNode(
                            ComponentNode::WrappedAssembledComponentNode(
                                WrappedAssembledComponentNode {
                                    data: WrappedAssembledComponentInstance::wrap(wrapped.data),
                                    rendered: wrapped.rendered,
                                },
                            ),
                        ));
                    }
                },
            }
        }

        (res_c, res_h)
    }

    fn compare_component_html(
        x: &ComponentTree<ThisComp, DemirootComp>,
        y: &Html<DemirootComp>,
    ) -> bool {
        match y {
            Html::ComponentNode(ComponentNode::PackedComponentNode(_))
            | Html::ComponentNode(ComponentNode::PrepackedComponentNode(_)) => match x {
                ComponentTree::ThisComp(_) => true,
                _ => false,
            },
            Html::ComponentNode(ComponentNode::WrappedAssembledComponentNode(_)) => match x {
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
        let (mapped, nodes) = match after {
            Html::ComponentNode(ComponentNode::PackedComponentNode(mut packed)) => {
                let assembled = match before {
                    ComponentTree::ThisComp(before_child) => packed.assemble(Some(before_child)),
                    _ => packed.assemble(None),
                };
                let (assembled, nodes) = Self::render_assembled(assembled, self.this_as_demiroot());
                let msgs = assembled.borrow_mut().load_lazy_cmd();
                for msg in msgs {
                    self.lazy_post(msg);
                }
                (ComponentTree::ThisComp(assembled), nodes)
            }
            Html::ComponentNode(ComponentNode::PrepackedComponentNode(mut prepacked)) => {
                let assembled = match before {
                    ComponentTree::ThisComp(before_child) => prepacked.assemble(Some(before_child)),
                    _ => prepacked.assemble(None),
                };

                let (assembled, nodes) = Self::render_assembled(assembled, self.this_as_demiroot());
                let msgs = assembled.borrow_mut().load_lazy_cmd();
                for msg in msgs {
                    self.lazy_post(msg);
                }
                (ComponentTree::ThisComp(assembled), nodes)
            }
            Html::ComponentNode(ComponentNode::WrappedAssembledComponentNode(wrapped)) => {
                let assembled = wrapped.data;
                let nodes = wrapped.rendered;
                let msgs = assembled.borrow_mut().load_lazy_cmd();
                for msg in msgs {
                    self.lazy_post(msg);
                }
                (ComponentTree::ThisComp(assembled), nodes)
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
                let mut before_children = match before {
                    ComponentTree::Fragment(before_children)
                    | ComponentTree::Element(before_children) => before_children,
                    _ => std::collections::VecDeque::new(),
                };
                let mut fragment = std::collections::VecDeque::new();
                let mut nodes = VecDeque::new();
                for child in children {
                    let before_child = before_children.pop_front().unwrap_or(ComponentTree::None);
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
                (ComponentTree::Element(fragment), vec![node].into())
            }
            Html::Fragment(afters) => {
                let mut mapped = VecDeque::new();
                let mut nodes = VecDeque::new();
                let mut befores = before.into_deq();

                for after in afters {
                    let before = befores.pop_front().unwrap_or(ComponentTree::None);
                    let (m, mut n) = self.render_html(before, after);

                    mapped.push_back(m);
                    nodes.append(&mut n);
                }

                (ComponentTree::Fragment(mapped), nodes)
            }
        };

        (mapped, nodes)
    }

    fn render_assembled<C: Component>(
        assembled: (
            Rc<RefCell<dyn AssembledChildComponent<DemirootComp = C>>>,
            Vec<Html<C>>,
        ),
        demiroot: Option<Weak<RefCell<dyn AssembledDemirootComponent<ThisComp = C>>>>,
    ) -> (
        Rc<RefCell<dyn AssembledChildComponent<DemirootComp = C>>>,
        VecDeque<Node>,
    ) {
        let children = assembled.1;
        let assembled = assembled.0;
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
}
