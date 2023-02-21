#![allow(non_snake_case, non_upper_case_globals)]

pub mod actions;
pub mod navigation;

#[cfg(feature = "components")]
pub mod components;

pub mod prelude {
    pub use dioxus_class::prelude::*;

    pub use crate::actions::button;
    pub use crate::actions::dropdown;

    pub use crate::navigation::menu;
    pub use crate::navigation::navbar;

    #[cfg(feature = "components")]
    pub use crate::components::prelude::*;
}