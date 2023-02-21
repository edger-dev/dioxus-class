use dioxus::prelude::*;
use crate::prelude::*;

#[derive(Props, Clone)]
pub struct ListProps<'a> {
    pub class: Option<&'a Class>,
    pub items: Vec<Element<'a>>,
}

pub fn List<'a>(cx: Scope<'a, ListProps<'a>>) -> Element {
    cx.render(rsx!(
        ul {
            class: cx.props.class,
            cx.props.items.iter().map(| item | rsx!{
                li {
                    item,
                }
            })
        }
    ))
}

pub fn ListC<'a>(cx: Scope<'a, ListProps<'a>>, class: Class) -> Element {
    cx.render(rsx!(
        ul {
            class: class + cx.props.class,
            cx.props.items.iter().map(| item | rsx!{
                li {
                    item,
                }
            })
        }
    ))
}
