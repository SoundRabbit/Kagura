use crate::dom::component::TaskResolver;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub struct Props {
    timeout: u32,
    header: Vec<(String, String)>,
}

pub struct Response {
    pub type_: web_sys::XmlHttpRequestResponseType,
    pub text: Option<String>,
    pub url: String,
    pub status: u16,
}

impl Props {
    pub fn new() -> Self {
        Props {
            timeout: 3000,
            header: vec![],
        }
    }

    pub fn set_timeout(mut self, t: u32) -> Self {
        self.timeout = t;
        self
    }

    pub fn set_header(mut self, header: impl Into<String>, value: impl Into<String>) -> Self {
        self.header.push((header.into(), value.into()));
        self
    }
}

pub fn request<Msg>(
    method: impl Into<String>,
    url: impl Into<String>,
    props: Props,
    mut handler: impl FnMut(Result<Response, JsValue>) -> Msg + 'static,
) -> Box<dyn FnOnce(TaskResolver<Msg>)>
where
    Msg: 'static,
{
    let method = method.into();
    let url = url.into();
    Box::new(move |resolver: TaskResolver<Msg>| {
        let mut resolver = Some(resolver);
        match web_sys::XmlHttpRequest::new() {
            Err(e) => {
                resolver.take().map(|r| r(handler(Err(e))));
            }
            Ok(xhr) => match xhr.open(&method, &url) {
                Err(e) => {
                    resolver.take().map(|r| r(handler(Err(e))));
                }
                Ok(_) => {
                    xhr.set_timeout(props.timeout);
                    for header in props.header {
                        let (header, value) = header;
                        if let Err(e) = xhr.set_request_header(&header, &value) {
                            resolver.take().map(|r| r(handler(Err(e))));
                            return;
                        }
                    }
                    let xhr = Rc::new(xhr);
                    let h = Closure::wrap(Box::new({
                        let xhr = Rc::clone(&xhr);
                        move || {
                            if xhr.ready_state() == 4 {
                                let text = xhr.response_text();
                                let status = xhr.status();
                                match text {
                                    Err(e) => {
                                        resolver.take().map(|r| r(handler(Err(e))));
                                    }
                                    Ok(text) => match status {
                                        Err(e) => {
                                            resolver.take().map(|r| r(handler(Err(e))));
                                        }
                                        Ok(status) => {
                                            let response = Response {
                                                type_: xhr.response_type(),
                                                text: text,
                                                url: xhr.response_url(),
                                                status: status,
                                            };
                                            resolver.take().map(|r| r(handler(Ok(response))));
                                        }
                                    },
                                }
                            }
                        }
                    }) as Box<dyn FnMut()>);
                    xhr.set_onreadystatechange(Some(h.as_ref().unchecked_ref()));
                    h.forget();
                    let _ = xhr.send();
                }
            },
        };
    }) as Box<dyn FnOnce(TaskResolver<Msg>)>
}

pub fn get<Msg>(
    url: impl Into<String>,
    props: Props,
    handler: impl FnMut(Result<Response, JsValue>) -> Msg + 'static,
) -> Box<dyn FnOnce(TaskResolver<Msg>)>
where
    Msg: 'static,
{
    request("GET", url, props, handler)
}

pub fn post<Msg>(
    url: impl Into<String>,
    props: Props,
    handler: impl FnMut(Result<Response, JsValue>) -> Msg + 'static,
) -> Box<dyn FnOnce(TaskResolver<Msg>)>
where
    Msg: 'static,
{
    request("POST", url, props, handler)
}

pub fn put<Msg>(
    url: impl Into<String>,
    props: Props,
    handler: impl FnMut(Result<Response, JsValue>) -> Msg + 'static,
) -> Box<dyn FnOnce(TaskResolver<Msg>)>
where
    Msg: 'static,
{
    request("PUT", url, props, handler)
}

pub fn delete<Msg>(
    url: impl Into<String>,
    props: Props,
    handler: impl FnMut(Result<Response, JsValue>) -> Msg + 'static,
) -> Box<dyn FnOnce(TaskResolver<Msg>)>
where
    Msg: 'static,
{
    request("DELETE", url, props, handler)
}
