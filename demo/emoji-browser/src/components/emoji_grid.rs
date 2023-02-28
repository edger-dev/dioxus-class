use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;

#[derive(Props, Clone)]
pub struct Props<'a> {
    pub children: Element<'a>,
}

pub fn view<'a>(cx: Scope<'a, Props<'a>>) -> Element {
    cx.render(rsx!{
        div {
            class: class!(flex flex_row flex_wrap justify_center gap_4 mx_2),
            &cx.props.children
        }
    })
}