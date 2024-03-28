#![allow(non_snake_case)]

pub use dioxus_class_internal; 
pub use dioxus_class_macro; 

pub use paste;

pub mod macros;

pub mod build;

pub mod prelude {
    pub use dioxus_class_internal::Class;
    pub use dioxus_class_macro::class;
}

pub mod ext {
    pub use paste::paste;
    pub use crate::constant;
    pub use crate::constant_internal;
}