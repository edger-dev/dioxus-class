use dioxus::prelude::*;

use fermi::use_read;

use dioxus_daisyui::prelude::*;

use crate::components::*;
use crate::app;


pub fn view(cx: Scope) -> Element {
    let all = use_read(cx, app::EMOJIS);
    let filter = use_read(cx, app::FILTER);
    let filtered = if filter.len() == 0 {
        Vec::new()
    } else {
        app::filter_emojis(filter.as_str())
    };
    /* showing both cases, since it's slow to rebuild the all list, will find some way for caching */
    let all_class = if filter.len() > 0 { class!(hidden) } else { Class::NONE };
    let filter_class = if filter.len() == 0 { class!(hidden) } else { Class::NONE };
    cx.render(rsx!(
        div {
            class: class!(w_screen h_screen),
            div {
                class: class!(fixed top_0 w_full z_10 bg_base_100 flex justify_center),
                emoji_search::view {
                    value: "",
                }
            }
            div {
                class: class!(bottom_0 h_full mt_16) + all_class,
                emoji_grid::view {
                    all.iter().map(|emoji| rsx!(
                        emoji_card::view {
                            alias: emoji.alias,
                            value: emoji.value,
                        }
                    ))
                }
            }
            div {
                class: class!(bottom_0 h_full mt_16) + filter_class,
                emoji_grid::view {
                    filtered.iter().map(|emoji| rsx!(
                        emoji_card::view {
                            alias: emoji.alias,
                            value: emoji.value,
                        }
                    ))
                }
            }
        }
    ))
}