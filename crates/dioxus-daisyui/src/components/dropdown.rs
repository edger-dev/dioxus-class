use dioxus::prelude::*;
use crate::prelude::*;

#[derive(Props, Clone)]
pub struct DropdownProps<'a> {
    pub class: Option<&'a Class>,
    pub children: Element<'a>,
    pub content: ItemProps<'a>,
}

pub fn Dropdown<'a>(cx: Scope<'a, DropdownProps<'a>>) -> Element {
    cx.render(rsx!(
        div {
            class: class!(dropdown) + cx.props.class,
            &cx.props.children,
            div {
                class: class!(dropdown_content) + cx.props.content.class,
                &cx.props.content.children
            }
        }
    ))
}