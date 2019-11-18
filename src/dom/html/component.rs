use crate::component;
use super::Html;
use std::any::Any;

pub trait Composable : component::Composable<Html<Box<dyn Any>>> {
    fn update(&mut self, msg: Box<dyn Any>) -> Cmd<Msg, Sub>;
}

/// the function whitch called when a task is finished.
type Resolver<Msg> = Box<dyn FnOnce(Msg)>;

/// Cmd
pub enum Cmd<Msg, Sub> {
    None,
    Sub(Sub),
    Task(Box<dyn FnOnce(Resolver<Msg>)>),
}

impl<Msg, Sub> Cmd<Msg, Sub> {
    pub fn none() -> Self {
        Cmd::None
    }

    pub fn sub(sub: Sub) -> Self {
        Cmd::Sub(sub)
    }

    pub fn task(task: impl FnOnce(Resolver<Msg>) + 'static) -> Self {
        Cmd::Task(Box::new(task))
    }
}

pub struct Component<Msg, State, Sub>
where
    Msg: 'static,
    State: 'static,
    Sub: 'static,
{
    state: State,
    init: Box<dyn FnOnce() -> (State, Msg)>,
    update: Box<dyn Fn(&mut State, Msg) -> Cmd<Msg, Sub>>,
    render: Box<dyn Fn(&State) -> Html<Msg>>,
    subscribe: Option<Box<dyn Fn(Sub) -> Box<dyn Any>>>,
}

impl<Msg, State, Sub> Component<Msg, State, Sub>
where
    Msg: 'static,
    State: 'static,
    Sub: 'static,
{
    pub fn new(
        init: impl FnOnce() -> (State, Msg),
        update: impl Fn(&mut State, Msg) -> Cmd<Msg, Sub> + 'static,
        render: impl Fn(&State) -> Html<Msg> + 'static,
    ) -> Self {
        Component {

        }
    }
}

impl<Msg, State, Sub> Composable for Component<Msg, State, Sub> {
    fn update(&mut self, msg: Box<dyn Any>) -> Cmd<Msg, Sub> {
        if let Ok(msg) = msg.downcast::<Msg>() {
            (self.update)(&mut self.state, *msg)
        }else {
            Cmd::none()
        }
    }
}