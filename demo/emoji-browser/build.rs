use std::env;
use std::path::Path;
use dioxus_daisyui::build::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=css/classes.rs");
    let current_dir = env::current_dir()?;
    let css_dir = Path::new(&current_dir).join("css");
    let classes_path = Path::new(&css_dir).join("classes.rs");
    check_classes(classes_path)
}