use crate::dom::component::Messenger;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn tick<Msg>(interval: i32, mut msg: Box<dyn FnMut() -> Msg>) -> Box<dyn FnMut(Messenger<Msg>)>
where
    Msg: 'static,
{
    Box::new(move |mut messenger: Messenger<Msg>| {
        let msg = msg;
        let a = Closure::wrap(Box::new(move || {
            messenger(msg());
        }) as Box<dyn FnMut()>);

        web_sys::window()
            .unwrap()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                a.as_ref().unchecked_ref(),
                interval,
            );

        a.forget();
    })
}
