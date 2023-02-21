use dioxus::prelude::*;
use crate::prelude::*;

pub fn Button<'a>(cx: Scope<'a, DivProps<'a>>) -> Element {
    cx.render(rsx!(
        button {
            class: Class::from(vec![button::btn]) + cx.props.class,
            &cx.props.children
        }
    ))
}