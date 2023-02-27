#![allow(non_snake_case)]

//! Example: Basic Tailwind usage
//!
//! This example shows how an app might be styled with TailwindCSS.
//!
//! To minify your tailwind bundle, currently you need to use npm. Follow these instructions:
//!
//!     https://dev.to/arctic_hen7/how-to-set-up-tailwind-css-with-yew-and-trunk-il9

use dioxus::prelude::*;
use dioxus_desktop::Config;

use dioxus_tailwindcss::prelude::*;
use dioxus_tailwindcss::ext::*;

constant!(body font);
constant!(title font);
constant!(items center);
constant!(flex grow);

fn main() {
    dioxus_desktop::launch_cfg(
        app,
        Config::new()
            .with_custom_head("<script src=\"https://cdn.tailwindcss.com\"></script>".to_string()),
    );
}

pub fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            header { class: class!(text_gray_400 bg_gray_900 body_font),
                div { class: class!(container mx_auto flex flex_wrap p_5 flex_col md(flex_row) items_center),
                    a { class: class!(flex title_font font_medium items_center text_white mb_4 md(mb_0)),
                        StacksIcon {}
                        span { class: class!("ml-3 text-xl"), "Hello Dioxus!"}
                    }
                    nav { class: class!(md(ml_auto) flex flex_wrap items_center text_base justify_center),
                        a { class: class!(mr_5 hover(text_white)), "First Link"}
                        a { class: class!(mr_5 hover(text_white)), "Second Link"}
                        a { class: class!(mr_5 hover(text_white)), "Third Link"}
                        a { class: class!(mr_5 hover(text_white)), "Fourth Link"}
                    }
                    button {
                        class: class!(inline_flex items_center bg_gray_800 border_0 py_1 px_3 focus(outline_none) hover(bg_gray_700) rounded text_base mt_4 md(mt_0)),
                        "Button"
                        RightArrowIcon {}
                    }
                }
            }

            section { class: class!(text_gray_400 bg_gray_900 body_font),
                div { class: class!(container mx_auto flex px_5 py_24 md(flex_row) flex_col items_center),
                    div { class: class!(lg(flex_grow) md(w_1__2) lg(pr_24) md(pr_16) flex flex_col md(items_start) md(text_left) mb_16 md(mb_0) items_center text_center),
                        h1 { class: class!(title_font sm(text_4xl) text_3xl mb_4 font_medium text_white),
                            br { class: class!(hidden lg(inline_block)) }
                            "Dioxus Sneak Peek"
                        }
                        p {
                            class: class!(mb_8 leading_relaxed),

                            "Dioxus is a new UI framework that makes it easy and simple to write cross-platform apps using web
                            technologies! It is functional, fast, and portable. Dioxus can run on the web, on the desktop, and
                            on mobile and embedded platforms."

                        }
                        div { class: class!(flex justify_center),
                            button {
                                class: class!(inline_flex text_white bg_indigo_500 border_0 py_2 px_6 focus(outline_none) hover(bg_indigo_600) rounded text_lg),
                                "Learn more"
                            }
                            button {
                                class: class!(ml_4 inline_flex text_gray_400 bg_gray_800 border_0 py_2 px_6 focus(outline_none) hover(bg_gray_700) hover(text_white) rounded text_lg),
                                "Build an app"
                            }
                        }
                    }
                    div { class: class!(lg(max_w_lg) lg(w_full) md(w_1__2) w_5__6),
                        img {
                            class: class!(object_cover object_center rounded),
                            src: "https://i.imgur.com/oK6BLtw.png",
                            referrerpolicy:"no-referrer",
                            alt: "hero",
                        }
                    }
                }
            }
        }
    ))
}

pub fn StacksIcon(cx: Scope) -> Element {
    cx.render(rsx!(
        svg {
            fill: "none",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            class: class!(w_10 h_10 text_white p_2 bg_indigo_500 rounded_full),
            view_box: "0 0 24 24",
            path { d: "M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"}
        }
    ))
}

pub fn RightArrowIcon(cx: Scope) -> Element {
    cx.render(rsx!(
        svg {
            fill: "none",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            class: class!(w_4 h_4 ml_1),
            view_box: "0 0 24 24",
            path { d: "M5 12h14M12 5l7 7-7 7"}
        }
    ))
}
