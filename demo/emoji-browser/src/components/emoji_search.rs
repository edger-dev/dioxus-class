use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;

use crate::app;

#[component]
pub fn view(value: &'static str) -> Element {
    rsx! {
        input {
            r#type: "text",
            class: class!(w_80 h_12 text_2xl m_4 px_2 input input_primary input_bordered),
            placeholder: "Type to search",
            oninput: move | evt | {
                *app::FILTER.write() = evt.value().clone();
            }
        }
    }
}