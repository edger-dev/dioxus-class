use crate::prelude::Class;
use std::fs::File;
use std::io::LineWriter;
use std::io::Write;
use std::path::Path;

pub struct Classes(Vec<Class>);

pub fn generate<P: AsRef<Path> + Clone>(
    path: P,
    classes: Classes,
) -> Result<(), Box<dyn std::error::Error>> {
    let path_str = path.clone().as_ref().to_string_lossy().to_string();
    let file = File::create(path)?;
    let mut writer = LineWriter::new(file);
    let mut size: usize = 0;
    for class in classes.0 {
        let str = class.to_class();
        size += str.len();
        writer.write_all(format!("<div class=\"{}\"/>\n", str).as_bytes())?;
    }
    println!(
        "cargo:warning=dioxus-class::build::generate: [{}] {}",
        size, path_str
    );
    Ok(())
}

impl From<Vec<Class>> for Classes {
    fn from(v: Vec<Class>) -> Self {
        Self(v)
    }
}

impl From<Vec<&Class>> for Classes {
    fn from(v: Vec<&Class>) -> Self {
        let mut classes = vec![];
        for class in v {
            classes.push(class.clone());
        }
        Self(classes)
    }
}