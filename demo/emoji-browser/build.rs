use std::env;
use std::path::Path;

use dioxus_daisyui::build::generate_classes;
use dioxus_daisyui::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=styles.rs");
    // let out_dir = env::var_os("OUT_DIR").unwrap();
    let current_dir = env::current_dir()?;
    let css_dir = Path::new(&current_dir).join("css");

    let classes_path = Path::new(&css_dir).join("classes.html");
    let classes = include!("classes.rs");
    generate_classes(classes_path, classes)?;
    
    dioxus_daisyui::build::npx_tailwindcss(css_dir, "tailwind.input.css", "../public/css/tailwind.css")
}