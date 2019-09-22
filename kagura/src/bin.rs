use crate::component::Component;
use crate::component::Composable;
use crate::native;
use crate::renderer::Renderer;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

thread_local!(static APP: RefCell<Option<App>> = RefCell::new(None));

struct App {
    root_component: Box<Composable>,
    renderer: Renderer,
}

pub fn run<M, S, B>(mut root_component: Component<M, S, B>, id: &str)
where
    M: 'static,
    S: 'static,
    B: 'static,
{
    let node = root_component.render(None);
    let root = native::get_element_by_id(id);
    let renderer = Renderer::new(node, root.into());
    let root_component: Box<Composable> = Box::new(root_component);
    APP.with(|app| {
        *app.borrow_mut() = Some(App {
            root_component,
            renderer,
        })
    });
}

pub fn update(mut id: u128, mut msg: Box<Any>) {
    APP.with(|app| {
        if let Some(app) = &mut (*app.borrow_mut()) {
            while let Some((new_msg, new_id)) = app.root_component.update(id, Rc::from(msg)) {
                msg = new_msg;
                id = new_id;
                native::console_log(&id.to_string());
            }
            let node = app.root_component.render(Some(id));
            app.renderer.update(node);
        }
    });
}
