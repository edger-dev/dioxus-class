use crate::prelude::Classes;
use std::fs::File;
use std::io::LineWriter;
use std::io::Write;
use std::path::Path;

pub fn write_file<P: AsRef<Path> + Clone>(
    path: P,
    content: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let path_str = path.clone().as_ref().to_string_lossy().to_string();
    let file = File::create(path)?;
    let mut writer = LineWriter::new(file);
    writer.write_all(content.as_bytes())?;
    println!(
        "cargo:warning=dioxus-class::build::write_file: [{}] {}",
        content.len(), path_str
    );
    Ok(())
}

pub fn generate<P: AsRef<Path> + Clone>(
    path: P,
    styles: Vec<Classes>,
) -> Result<(), Box<dyn std::error::Error>> {
    let path_str = path.clone().as_ref().to_string_lossy().to_string();
    let file = File::create(path)?;
    let mut writer = LineWriter::new(file);
    let mut style_count: usize = 0;
    let mut class_count: usize = 0;
    let mut size: usize = 0;
    for style in styles {
        style_count += 1;
        writer.write_all(format!("<!-- {} -->\n", style.name).as_bytes())?;
        for (class_name, class) in style.classes.iter() {
            let str = class.to_class();
            class_count += 1;
            size += str.len();
            writer.write_all(format!("<div class=\"{}\">{}</div>\n", str, class_name).as_bytes())?;
        }
    }
    println!(
        "cargo:warning=dioxus-class::build::generate: {}-{}-[{}] {}",
        style_count, class_count, size, path_str
    );
    Ok(())
}