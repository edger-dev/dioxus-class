pub use dioxus_class::build::*;

use std::path::Path;
use std::process::Command;

pub fn tailwindcss<P: AsRef<Path> + Clone>(
    path: P,
    input: &str,
    output: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new("tailwindcss");
    cmd.current_dir(path);
    let result = cmd
        .arg("-i").arg(input)
        .arg("-o").arg(output).output()?;
    if !result.status.success() {
        Err(format!("tailwindcss failed: {} {} -> {:?}", input, output, result).into())
    } else {
        println!("cargo:warning=dioxus-tailwind::build::tailwindcss: {} {}", input, output);
        let stdout = String::from_utf8(result.stdout)?;
        for line in stdout.lines() {
            println!("cargo:warning=    {:#?}", line);
        }
        Ok(())
    }
}

pub fn npx_tailwindcss<P: AsRef<Path> + Clone>(
    path: P,
    input: &str,
    output: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new("npx");
    cmd.current_dir(path);
    let result = cmd
        .arg("tailwindcss")
        .arg("-i").arg(input)
        .arg("-o").arg(output).output()?;
    if !result.status.success() {
        Err(format!("tailwindcss failed: {} {} -> {:?}", input, output, result).into())
    } else {
        println!("cargo:warning=dioxus-tailwind::build::npx_tailwindcss: {} {}", input, output);
        let stdout = String::from_utf8(result.stdout)?;
        for line in stdout.lines() {
            println!("cargo:warning=    {:#?}", line);
        }
        Ok(())
    }
}
