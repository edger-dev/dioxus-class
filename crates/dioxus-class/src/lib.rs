#![allow(non_snake_case)]

pub use paste;

pub mod class;
pub mod classes;

pub mod macros;

pub mod components;

pub mod build;

pub mod prelude {
    pub use crate::class::Class;
    pub use crate::classes::Classes;

    pub use crate::components::prelude::*;

    pub use crate::class;
    pub use crate::style_type;
    pub use crate::style;
}

pub mod ext {
    pub use paste::paste;
    pub use crate::constant;
    pub use crate::constant_internal;
}