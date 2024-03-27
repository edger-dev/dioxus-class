use dioxus::prelude::*;

use dioxus_daisyui::prelude::*;

use crate::components::*;
use crate::app;

#[component]
pub fn Home() -> Element {
    let all: &Vec<emoji_card::Props> = &app::EMOJIS();
    let filter: &str = &app::FILTER();
    let filtered = if filter.len() == 0 {
        Vec::new()
    } else {
        app::filter_emojis(filter)
    };
    let shared_class = class!(bottom_0 h_full mt_20);
    let all_class = if filter.len() > 0 { class!(hidden) } else { Class::NONE };
    let filter_class = if filter.len() == 0 { class!(hidden) } else { Class::NONE };
    rsx! {
        div {
            class: class!(w_screen h_screen),
            div {
                class: class!(fixed top_0 w_full z_10 bg_base_100 flex justify_center),
                emoji_search::view {
                    value: "",
                }
            }
            div {
                class: all_class + &shared_class,
                emoji_grid::view {
                    for emoji in all {
                        emoji_card::view {
                            alias: emoji.alias,
                            value: emoji.value,
                        }
                    }
                }
            }
            div {
                class: filter_class + &shared_class,
                emoji_grid::view {
                    for emoji in filtered {
                        emoji_card::view {
                            alias: emoji.alias,
                            value: emoji.value,
                        }
                    }
                }
            }
        }
    }
}