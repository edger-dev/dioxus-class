#![allow(non_snake_case, non_upper_case_globals)]

pub use dioxus_class;
pub use dioxus_class::build;

pub mod responsive;
pub mod modifier;
pub mod layout;
pub mod flex;
pub mod spacing;
pub mod sizing;
pub mod typography;

pub mod components;

pub mod prelude {
    pub use dioxus_class::prelude::*;

    pub use crate::responsive::*;
    pub use crate::modifier::*;
    pub use crate::layout::*;
    pub use crate::flex::*;
    pub use crate::spacing::*;
    pub use crate::sizing::*;
    pub use crate::typography::*;

    pub use crate::components::prelude::*;
}