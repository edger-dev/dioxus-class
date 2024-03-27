use dioxus::prelude::*;
use emoji_browser::app::App;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    launch(App);
}