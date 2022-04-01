use kagura::prelude::*;
use nusa::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};

pub struct Props {}

pub enum Msg {
    RenderCanvas(web_sys::Node),
}

pub enum On {}

pub struct Refer {}

impl Component for Refer {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for Refer {}

impl Constructor for Refer {
    fn constructor(_props: Self::Props) -> Self {
        Self {}
    }
}

impl Update for Refer {
    fn update(self: Pin<&mut Self>, msg: Self::Msg) -> Cmd<Self> {
        match msg {
            Msg::RenderCanvas(node) => {
                if let Some(canvas) = node.dyn_into::<web_sys::HtmlCanvasElement>().ok() {
                    let context = canvas.get_context("2d").unwrap().unwrap();
                    let context = context
                        .dyn_into::<web_sys::CanvasRenderingContext2d>()
                        .unwrap();
                    context.set_fill_style(&JsValue::from("green"));
                    context.fill_rect(10.0, 10.0, 150.0, 100.0);
                    web_sys::console::log_1(&JsValue::from("render canvas"));
                }
                Cmd::none()
            }
        }
    }
}

impl Render<Html> for Refer {
    type Children = ();
    fn render(&self, _children: Self::Children) -> Html {
        Html::canvas(
            Attributes::new(),
            Events::new().refer(self, |node| Msg::RenderCanvas(node)),
            vec![],
        )
    }
}
