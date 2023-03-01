use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;
use fermi::prelude::*;

use crate::app;

#[derive(Props, Clone)]
pub struct Props<'a> {
    pub value: &'a str,
}

pub fn view<'a>(cx: Scope<'a, Props<'a>>) -> Element {
    let set_filter = use_set(cx, app::FILTER);
    cx.render(rsx!{
        input {
            r#type: "text",
            class: class!(w_80 h_12 text_2xl m_4 px_2 input input_primary input_bordered),
            placeholder: "Type to search",
            oninput: move | evt | {
                set_filter(evt.value.clone());
            }
        }
    })
}