use dioxus::prelude::*;
use crate::prelude::*;

pub fn Menu<'a>(cx: Scope<'a, ListProps<'a>>) -> Element {
    ListC(cx, Class::from(vec![menu::menu]))
}