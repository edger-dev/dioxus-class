use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;

#[component]
pub fn view(children: Element) -> Element {
    rsx! {
        div {
            class: class!(flex flex_row flex_wrap justify_center gap_4 mx_2),
            { children }
        }
    }
}