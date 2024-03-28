use crate::prelude::*;
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

pub fn generate_classes<P: AsRef<Path> + Clone>(
    path: P,
    classes: Vec<Class>,
) -> Result<(), Box<dyn std::error::Error>> {
    let path_str = path.clone().as_ref().to_string_lossy().to_string();
    let file = File::create(path)?;
    let mut writer = LineWriter::new(file);
    let mut class_count: usize = 0;
    let mut size: usize = 0;
    for class in classes {
        let str = class.to_class();
        class_count += 1;
        size += str.len();
        writer.write_all(format!("<div class=\"{}\">{}</div>\n", str, class.to_string()).as_bytes())?;
    }
    println!(
        "cargo:warning=dioxus-class::build::generate_classes: {}-[{}] {}",
        class_count, size, path_str
    );
    Ok(())
}
