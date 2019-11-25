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
    url: &str,
    mut msg_gen: impl FnMut() -> Msg + 'static,
) -> Box<dyn FnOnce(Messenger<Msg>)>
where
    Msg: 'static,
{
    Box::new(move |mut messenger: Messenger<Msg>| {})
}

pub fn on_error<Msg>(
    url: &str,
    mut msg_gen: impl FnMut() -> Msg + 'static,
) -> Box<dyn FnOnce(Messenger<Msg>)>
where
    Msg: 'static,
{
    Box::new(move |mut messenger: Messenger<Msg>| {})
}

pub fn on_message<Msg>(
    url: &str,
    mut msg_gen: impl FnMut() -> Msg + 'static,
) -> Box<dyn FnOnce(Messenger<Msg>)>
where
    Msg: 'static,
{
    Box::new(move |mut messenger: Messenger<Msg>| {})
}

pub fn on_open<Msg>(
    url: &str,
    mut msg_gen: impl FnMut() -> Msg + 'static,
) -> Box<dyn FnOnce(Messenger<Msg>)>
where
    Msg: 'static,
{
    Box::new(move |mut messenger: Messenger<Msg>| {})
}
