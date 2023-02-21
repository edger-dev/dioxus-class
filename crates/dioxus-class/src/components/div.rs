use dioxus::prelude::*;
use crate::prelude::*;

#[derive(Props, Clone)]
pub struct DivProps<'a> {
    pub class: Option<&'a Class>,
    pub children: Element<'a>,
}

pub fn Div<'a>(cx: Scope<'a, DivProps<'a>>) -> Element {
    cx.render(rsx!(
        div {
            class: cx.props.class,
            &cx.props.children
        }
    ))
}

pub fn DivC<'a>(cx: Scope<'a, DivProps<'a>>, class: Class) -> Element {
    cx.render(rsx!(
        div {
            class: class + cx.props.class,
            &cx.props.children
        }
    ))
}