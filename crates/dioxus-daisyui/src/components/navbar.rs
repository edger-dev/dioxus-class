use dioxus::prelude::*;
use crate::prelude::*;

#[derive(Props, Clone)]
pub struct NavbarProps<'a> {
    pub class: Option<&'a Class>,

    pub start: Option<DivProps<'a>>,
    pub center: Option<DivProps<'a>>,
    pub end: Option<DivProps<'a>>,
}

pub fn Navbar<'a>(cx: Scope<'a, NavbarProps<'a>>) -> Element {
    cx.render(rsx!(
        div {
            class: Class::from(vec![navbar::navbar]) + cx.props.class,
            if let Some(start) = &cx.props.start {
                rsx!(div {
                    class: Class::from(vec![navbar::navbar_start]) + start.class,
                    &start.children
                })
            }
            if let Some(center) = &cx.props.center {
                rsx!(div {
                    class: Class::from(vec![navbar::navbar_center]) + center.class,
                    &center.children
                })
            }
            if let Some(end) = &cx.props.end {
                rsx!(div {
                    class: Class::from(vec![navbar::navbar_end]) + end.class,
                    &end.children,
                })
            }
        }
    ))
}