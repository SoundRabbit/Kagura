use crate::dom::component::Messenger;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

thread_local!(static WEB_SOCKETS: RefCell<HashMap<&'static str, Rc<web_sys::WebSocket>>> = RefCell::new(HashMap::new()));

fn web_socket(url: &'static str) -> Rc<web_sys::WebSocket> {
    WEB_SOCKETS.with(|web_sockets| {
        if let Some(ws) = web_sockets.borrow().get(url) {
            Rc::clone(ws)
        } else {
            let ws = Rc::new(web_sys::WebSocket::new(url).unwrap());
            web_sockets.borrow_mut().insert(url, Rc::clone(&ws));
            ws
        }
    })
}

pub fn on_close<Msg>(
    url: &'static str,
    mut msg_gen: impl FnMut() -> Msg + 'static,
) -> Box<dyn FnOnce(Messenger<Msg>)>
where
    Msg: 'static,
{
    Box::new(move |mut messenger: Messenger<Msg>| {
        let socket = web_socket(url);
        let a = Closure::wrap(Box::new(move || {
            messenger(msg_gen());
        }) as Box<dyn FnMut()>);
        socket.set_onclose(Some(a.as_ref().unchecked_ref()));
        a.forget();
    })
}

pub fn on_error<Msg>(
    url: &'static str,
    mut msg_gen: impl FnMut() -> Msg + 'static,
) -> Box<dyn FnOnce(Messenger<Msg>)>
where
    Msg: 'static,
{
    Box::new(move |mut messenger: Messenger<Msg>| {
        let socket = web_socket(url);
        let a = Closure::wrap(Box::new(move || {
            messenger(msg_gen());
        }) as Box<dyn FnMut()>);
        socket.set_onerror(Some(a.as_ref().unchecked_ref()));
        a.forget();
    })
}

pub fn on_message<Msg>(
    url: &'static str,
    mut msg_gen: impl FnMut() -> Msg + 'static,
) -> Box<dyn FnOnce(Messenger<Msg>)>
where
    Msg: 'static,
{
    Box::new(move |mut messenger: Messenger<Msg>| {
        let socket = web_socket(url);
        let a = Closure::wrap(Box::new(move || {
            messenger(msg_gen());
        }) as Box<dyn FnMut()>);
        socket.set_onmessage(Some(a.as_ref().unchecked_ref()));
        a.forget();
    })
}

pub fn on_open<Msg>(
    url: &'static str,
    mut msg_gen: impl FnMut() -> Msg + 'static,
) -> Box<dyn FnOnce(Messenger<Msg>)>
where
    Msg: 'static,
{
    Box::new(move |mut messenger: Messenger<Msg>| {
        let socket = web_socket(url);
        let a = Closure::wrap(Box::new(move || {
            messenger(msg_gen());
        }) as Box<dyn FnMut()>);
        socket.set_onopen(Some(a.as_ref().unchecked_ref()));
        a.forget();
    })
}
