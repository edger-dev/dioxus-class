#![allow(non_snake_case)]

pub mod class;

#[cfg(feature = "components")]
pub mod components;

pub mod prelude {
    pub use crate::class::Class;

    #[cfg(feature = "components")]
    pub use crate::components::prelude::*;
}