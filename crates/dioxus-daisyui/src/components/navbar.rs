use dioxus::prelude::*;
use crate::prelude::*;

#[derive(Props, Clone)]
pub struct NavbarProps<'a> {
    pub class: Option<&'a Class>,

    pub start: Option<ItemProps<'a>>,
    pub center: Option<ItemProps<'a>>,
    pub end: Option<ItemProps<'a>>,
}

pub fn Navbar<'a>(cx: Scope<'a, NavbarProps<'a>>) -> Element {
    cx.render(rsx!(
        div {
            class: class!(navbar::navbar) + cx.props.class,
            if let Some(start) = &cx.props.start {
                rsx!(div {
                    class: class!(navbar::navbar_start) + start.class,
                    &start.children
                })
            }
            if let Some(center) = &cx.props.center {
                rsx!(div {
                    class: class!(navbar::navbar_center) + center.class,
                    &center.children
                })
            }
            if let Some(end) = &cx.props.end {
                rsx!(div {
                    class: class!(navbar::navbar_end) + end.class,
                    &end.children,
                })
            }
        }
    ))
}