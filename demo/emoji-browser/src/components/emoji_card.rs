use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;

#[derive(Props, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Props {
    pub alias: &'static str,
    pub value: &'static str,
}

#[component]
pub fn view(alias: &'static str, value: &'static str) -> Element {
    rsx! {
        div {
            class: class!(card card_compact w_64 h_64 bg_base_300 shadow_xl text_center hover(bg_base_200) hover(scale_105)),
            div {
                class: class!(text_8xl py_10),
                "{value}",
            },
            div {
                class: class!(card_body text_center items_center),
                div {
                    class: class!(card_title text_sm text_base_content),
                    "{alias}",
                }
            }
        }
    }
}