#![allow(non_snake_case)]

pub mod class;
pub mod classes;

pub mod macros;

pub mod components;

pub mod build;

pub mod prelude {
    pub use crate::class::Class;
    pub use crate::classes::Classes;

    // macros
    pub use crate::class;
    pub use crate::style;
    pub use crate::style_type;

    pub use crate::components::prelude::*;
}