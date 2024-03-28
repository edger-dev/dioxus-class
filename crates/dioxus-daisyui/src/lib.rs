#![allow(non_snake_case, non_upper_case_globals)]

#![doc = include_str!("../README.md")]

pub use dioxus_tailwindcss;
pub use dioxus_tailwindcss::dioxus_class;

pub use dioxus_tailwindcss::build;

pub mod macros;
pub mod colors;

pub mod actions;
pub mod display;
pub mod input;
pub mod layout;

pub mod navigation;
pub mod mockup;

pub mod prelude {
    pub use dioxus_tailwindcss::prelude::*;

    pub use crate::colors::*;

    pub use crate::actions::*;
    pub use crate::display::*;
    pub use crate::input::*;
    pub use crate::layout::*;
    pub use crate::navigation::*;
    pub use crate::mockup::*;
}

pub mod ext {
    pub use dioxus_tailwindcss::ext::*;

    pub use crate::semantic_colors;
}