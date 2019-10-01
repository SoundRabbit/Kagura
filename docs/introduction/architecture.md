#  Architecture

The architecture of Kagura is inspired by The Elm Architecture and Halogen of PureScript.

Each component is constructed of **initial_state**, **update** and **render**. **intial_state** is initial state of own component. **update** is a function which updates state with messgae and returns `Cmd<Msg, Sub>`. **render** is a fuction which constructs virtual-dom with the state of own component, and **render** can send message to **update** by event-listener.

<figure>
    ![architecture](/Kagura/docs/img/architecture.png)
    <figcaption>the architecture of Kagura-component</figcaption>
</figure>

[back to index](https://soundrabbit.github.io/Kagura/)
