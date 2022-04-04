//! # Examples
//!
//! ```
//! use kagura::prelude::*;
//! use tokio::time;
//!
//! struct Props {
//!     initial_count: usize,
//! }
//!
//! enum Msg {
//!     SetCount(usize)
//!     WaitCount,
//! }
//!
//! enum On {}
//!
//! struct MyComponent {
//!     count: usize,
//! }
//!
//! impl Component for MyComponent {
//!     type Props = Props;
//!     type Msg = Msg;
//!     type Event = On;
//! }
//!
//! impl Constructor for MyComponent {
//!     fn constructor(props: Props) -> Self {
//!         Self { count: props.initial_count }
//!     }
//! }
//!
//! impl Update for MyComponent {
//!     fn on_assemble(self: Pin<&mut Self>) -> Cmd<Self> {
//!         Cmd::chain(Msg::WaitCount)
//!     }
//!
//!     fn on_load(self: Pin<&mut Self>, props: Props) -> Cmd<Self> {
//!         if self.count < props.initial_count {
//!             self.count = props.initial_count;
//!         }
//!         Cmd::none()
//!     }
//!
//!     fn update(self: Pin<&mut Self>, msg: Msg) -> Cmd<Self> {
//!         match msg {
//!             Msg::SetCount(count) => {
//!                 self.count = count;
//!                 Cmd::none()
//!             }
//!             Msg::WaitCount => {
//!                 let count = self.count;
//!                 Cmd::task(async move {
//!                     time::sleep(Duration::from_millis(1000)).await;
//!                     Cmd::chain(Msg::SetCount(count + 1))
//!                 })
//!             }
//!         }
//!     }
//! }
//!
//! impl Render<String> for MyComponent {
//!     type Children = (String, String);
//!     fn render(&self, (prefix, suffix): Self::Children) -> String {
//!         format!("{}{}{}", prefix, self.count, suffix)
//!     }
//! }
//! ```

mod cmd;

use std::pin::Pin;

pub use cmd::Cmd;

/// `Component` trait is a common component in Kagura-component.  
/// In this trait, tyoes of `Props`, `Msg` and `Event` are defined.
///
pub trait Component: Sized {
    /// The type of property of this component.  
    /// Parent component gives a property to this component. The property will be used to generate initial state of this component.
    type Props;

    /// The type of message of this component.  
    /// A message transfers message of an event and so on occurs in this component to `update` function.
    type Msg;

    /// The type of event of this component.  
    /// An event transfers message to parent component.
    type Event;
}

/// A constructor of the component is defined.  
/// The Constructor generates a component with `Props`.
pub trait Constructor: Component {
    /// The constructor of a component.
    ///
    /// # Arguments
    ///
    /// - `props` - A property which given by parent component to generate this component.
    fn constructor(_props: Self::Props) -> Self;
}

pub trait Update: Component {
    fn on_assemble(self: Pin<&mut Self>) -> Cmd<Self> {
        Cmd::None
    }
    fn on_load(self: Pin<&mut Self>, _props: Self::Props) -> Cmd<Self> {
        Cmd::None
    }
    fn update(self: Pin<&mut Self>, _msg: Self::Msg) -> Cmd<Self> {
        Cmd::None
    }
}

pub trait Render<T>: Component {
    type Children: Default;
    fn render(&self, children: Self::Children) -> T;
}
