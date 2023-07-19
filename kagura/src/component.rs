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
//! }
//!
//! enum On {}
//!
//! struct Timer {
//!     interval: u64,
//! }
//!
//! struct MyComponent {
//!     count: usize,
//! }
//!
//! impl BatchProcess<MyComponent> for Timer {
//!     fn poll(&mut self) -> Pin<Box<dyn Future<Output = Cmd<MyComponent>>>> {
//!         let duration = Duration::from_millis(self.interval);
//!         Box::pin(async move {
//!             time::sleep(duration).await;
//!             Cmd::chain(Msg::SetCount(count + 1))
//!         })
//!     }
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
//!         Cmd::batch(Timer {
//!             interval: 1000
//!         })
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

pub mod cmd;

use std::pin::Pin;

pub use cmd::BatchProcess;
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
