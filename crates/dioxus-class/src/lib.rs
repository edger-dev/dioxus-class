#![allow(non_snake_case)]

pub mod class;

pub mod macros;

#[cfg(feature = "components")]
pub mod components;

#[cfg(feature = "build")]
pub mod build;

pub mod prelude {
    pub use crate::class::Class;
    pub use crate::class;

    #[cfg(feature = "components")]
    pub use crate::components::prelude::*;

    #[cfg(feature = "build")]
    pub use crate::build;
}