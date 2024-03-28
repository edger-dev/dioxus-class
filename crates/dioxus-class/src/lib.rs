#![allow(non_snake_case)]

pub use paste;

pub mod class;

pub mod macros;

pub mod build;

pub mod prelude {
    pub use crate::class::Class;
}

pub mod ext {
    pub use paste::paste;
    pub use crate::constant;
    pub use crate::constant_internal;
}