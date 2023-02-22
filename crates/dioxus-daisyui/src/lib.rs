#![allow(non_snake_case, non_upper_case_globals)]

pub use dioxus_tailwindcss;
pub use dioxus_tailwindcss::dioxus_class;
pub use dioxus_tailwindcss::dioxus_class::build;

pub mod actions;
pub mod display;
pub mod input;
pub mod layout;

pub mod navigation;
pub mod mockup;

pub mod components;

pub mod prelude {
    pub use dioxus_tailwindcss::prelude::*;

    pub use crate::actions::button;
    pub use crate::actions::dropdown;
    pub use crate::actions::modal;
    pub use crate::actions::swap;

    pub use crate::display::label;
    pub use crate::display::alert;
    pub use crate::display::avatar;
    pub use crate::display::badge;
    pub use crate::display::card;
    pub use crate::display::carousel;
    pub use crate::display::chat;
    pub use crate::display::collapse;
    pub use crate::display::countdown;
    pub use crate::display::kbd;
    pub use crate::display::progress;
    pub use crate::display::radial_progress;
    pub use crate::display::stat;
    pub use crate::display::table;
    pub use crate::display::tooltip;

    pub use crate::input::form;
    pub use crate::input::checkbox;
    pub use crate::input::file_input;
    pub use crate::input::radio;
    pub use crate::input::range;
    pub use crate::input::rating;
    pub use crate::input::select;
    pub use crate::input::input;
    pub use crate::input::textarea;
    pub use crate::input::toggle;

    pub use crate::layout::artboard;
    pub use crate::layout::button_group;
    pub use crate::layout::divider;
    pub use crate::layout::drawer;
    pub use crate::layout::footer;
    pub use crate::layout::hero;
    pub use crate::layout::indicator;
    pub use crate::layout::input_group;
    pub use crate::layout::mask;
    pub use crate::layout::stack;
    pub use crate::layout::toast;

    pub use crate::navigation::breadcrumbs;
    pub use crate::navigation::bottom_navigation;
    pub use crate::navigation::link;
    pub use crate::navigation::menu;
    pub use crate::navigation::navbar;
    pub use crate::navigation::steps;
    pub use crate::navigation::tab;

    pub use crate::mockup;

    pub use crate::components::prelude::*;
}