use crate::component::Component;
use crate::component::Composable;
use crate::native;
use crate::renderer::Renderer;
use std::any::Any;
use std::cell::RefCell;

static mut APP: Option<(Box<Composable>, Renderer)> = None;

pub fn run<M, S, B>(mut component: Component<M, S, B>, id: &str)
where
    M: 'static,
    S: 'static,
    B: 'static,
{
    let node = component.render(None);
    let root = native::get_element_by_id(id);
    let renderer = Renderer::new(node, root.into());
    let composable: Box<Composable> = Box::new(component);
    unsafe {
        APP = Some((composable, renderer));
    }
}

pub fn update(id: u128, msg: &Any) {
    unsafe {
        if let Some((app, renderer)) = &mut APP {
            if app.update(id, msg) {
                let node = app.render(Some(id));
                renderer.update(node);
            }
        }
    }
}
