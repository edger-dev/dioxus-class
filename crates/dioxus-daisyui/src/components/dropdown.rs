use dioxus::prelude::*;
use crate::prelude::*;

#[derive(Props, Clone)]
pub struct DropdownProps<'a> {
    pub class: Option<&'a Class>,
    pub children: Element<'a>,
    pub content: DivProps<'a>,
}

pub fn Dropdown<'a>(cx: Scope<'a, DropdownProps<'a>>) -> Element {
    cx.render(rsx!(
        div {
            class: Class::from(vec![dropdown::dropdown]) + cx.props.class,
            &cx.props.children,
            div {
                class: Class::from(vec![dropdown::dropdown_content]) + cx.props.content.class,
                &cx.props.content.children
            }
        }
    ))
}