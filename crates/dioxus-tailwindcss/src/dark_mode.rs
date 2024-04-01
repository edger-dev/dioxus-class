use crate::ext::*;

// https://tailwindcss.com/docs/dark-mode
/// dark:
pub fn dark(v: &str) -> String {
    "dark:".to_owned() + v
}

// Can not have both modifier and constant with same name, so adding an extra understore
constant!(dark_);

