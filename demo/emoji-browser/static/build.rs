use std::env;
use std::path::Path;

use dioxus::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // println!("cargo:rerun-if-changed=src/style.rs");
    // let out_dir = env::var_os("OUT_DIR").unwrap();
    let current_dir = env::current_dir()?;
    let pages_dir = Path::new(&current_dir).join("pages");

    let path = Path::new(&pages_dir).join("index.html");
    let mut app = VirtualDom::new(emoji_browser::app::App);
    _ = app.rebuild();
    let content = dioxus_ssr::render(&app);
    dioxus_daisyui::build::write_file(path, &content)?;

    let css_dir = Path::new(&current_dir).join("css");
    dioxus_daisyui::build::tailwindcss(css_dir, "tailwind.input.css", "../../public/css/tailwind.css")
}