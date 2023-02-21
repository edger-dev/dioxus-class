use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::io::LineWriter;
use crate::prelude::Class;

pub fn generate<P: AsRef<Path> + Clone>(path: P, classes: impl Iterator<Item = Class>) -> Result<(), Box<dyn std::error::Error>> {
    let path_str = path.clone().as_ref().to_string_lossy().to_string();
    let file = File::create(path)?;
    let mut writer = LineWriter::new(file);
    let mut size: usize = 0;
    for class in classes {
        let str = class.to_class();
        size += str.len();
        writer.write_all(format!("<div class=\"{}\"/>\n", str).as_bytes())?;
    }
    println!("cargo:warning=dioxus-class::build::generate: [{}] {}", size, path_str);
    Ok(())
}