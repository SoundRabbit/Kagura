use crate::node::Msg;
use std::future::Future;
use std::pin::Pin;

pub type Task = Pin<Box<dyn Future<Output = Vec<Msg>>>>;
