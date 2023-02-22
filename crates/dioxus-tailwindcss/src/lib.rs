#![allow(non_snake_case, non_upper_case_globals)]

pub use dioxus_class;
pub use dioxus_class::build;

pub mod components;

pub mod prelude {
    pub use dioxus_class::prelude::*;

    pub use crate::components::prelude::*;
}