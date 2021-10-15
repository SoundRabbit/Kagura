use super::*;

pub enum Cmd<C: Component> {
    None,
    Sub(C::Sub),
    Task(Box<dyn FnOnce(TaskResolver<C::Msg>)>),
    Batch(Box<dyn FnOnce(BatchResolver<C::Msg>)>),
    List(Vec<Self>),
}

impl<C: Component> Cmd<C> {
    /// Generates Cmd which means nothing to do
    pub fn none() -> Self {
        Self::None
    }

    /// Generates Cmd to send Component::Sub
    pub fn sub(on: C::Sub) -> Self {
        Self::Sub(on)
    }

    /// Task behave like Promise in JS.
    ///
    /// ```rust
    /// fn update(&mut self, props: &Props, msg: Msg) -> Cmd<Self> {
    ///     a_short_time_task();
    ///     a_short_time_task();
    ///     a_short_time_task();
    ///     Cmd::task(|resolve| {
    ///         let res = a_long_time_task();
    ///         resolve(Msg::Finish(res));
    ///     })
    /// }
    /// ```
    pub fn task(task: impl FnOnce(TaskResolver<C::Msg>) + 'static) -> Self {
        Self::Task(Box::new(task))
    }

    /// Batch behave like a setter of event-handler.
    /// Unlike TaskResolver, BatchResolver can be called more than once.
    ///
    /// ```
    /// Cmd::batch(|mut handle| {
    ///     let a = Closure::wrap(Box::new(move || {
    ///         handle(Msg::WindowIsResized);
    ///     }) as Box<dyn FnMut()>);
    ///     web_sys::window()
    ///         .unwrap()
    ///         .set_onresize(Some(a.as_ref().unchecked_ref()));
    ///     a.forget();
    /// })
    /// ```
    pub fn batch(batch: impl FnOnce(BatchResolver<C::Msg>) + 'static) -> Self {
        Self::Batch(Box::new(batch))
    }

    /// Reupdate by msg **after** rendering.
    /// If you want to chain msg before rendering, you can use recursive call with update.
    pub fn chain(msg: C::Msg) -> Self {
        Self::task(|resolve| resolve(msg))
    }

    /// Listed Cmds, which is evaluted in order.
    pub fn list(cmd_list: Vec<Self>) -> Self {
        Self::List(cmd_list)
    }
}
