use std::env;
use std::{fs::File, io::LineWriter, sync::RwLock};
use std::io::Write;

use lazy_static::lazy_static;

pub const DIOXUS_CLASS_BUILD_PATH: &'static str = "DIOXUS_CLASS_BUILD_PATH";

lazy_static! {
    static ref WRITER: RwLock<LineWriter<File>> = RwLock::new(create_writer().unwrap());
}

fn create_writer() -> Result<LineWriter<File>, Box<dyn std::error::Error>> {
    let path = match env::var_os(DIOXUS_CLASS_BUILD_PATH) {
        Some(build_path) => {
            println!("cargo:warning=dioxus-class-macro::build: {} = {:?}", DIOXUS_CLASS_BUILD_PATH, build_path);
            std::path::Path::new(&build_path).into()
        },
        None => {
            let mut path = std::env::current_dir()?.clone();
            path.push("classes");
            path.set_extension("rs");
            path
        },
    };
    println!("cargo:warning=dioxus-class-macro::build::create_writer(): {:?}", path);
    let file = File::create(path)?;
    let mut writer = LineWriter::new(file);
    writer.write_all("vec![\n\n".as_bytes())?;
    writer.flush()?;
    Ok(writer)
}

pub fn write_bytes(text: &str, bytes: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    WRITER.write().unwrap().write_all(bytes)?;
    WRITER.write().unwrap().flush()?;
    println!("cargo:warning=dioxus-class-macro::build::write_bytes(): [{}] {}", bytes.len(), text);
    Ok(())
}

pub fn write_text(text: &str, lines: &str) -> Result<(), Box<dyn std::error::Error>> {
    write_bytes(text, lines.as_bytes())
}