use std::any::Any;
use crate::dom::native;
use crate::component::Component;
use crate::component::Composable;

static mut APP: Option<(Box<Composable>, native::Renderer)> = None;

pub fn run<M, S, B>(mut component: Component<M, S, B>, id: &str)
where
    M: 'static,
    S: 'static,
    B: 'static,
{
    let node = component.render(None);
    let root = native::get_element_by_id(id);
    let renderer = native::Renderer::new(node, root.into());
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