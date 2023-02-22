use dioxus::prelude::*;
use crate::prelude::*;

#[derive(Props, Clone)]
pub struct ItemProps<'a> {
    pub class: Option<&'a Class>,
    pub children: Element<'a>,
}