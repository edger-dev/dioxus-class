# dioxus-class

Most dioxus samples are using plain strings for classes, e.g.

```rust
    button {
        class: "inline-flex items-center bg-gray-800 border-0 py-1 px-3 focus:outline-none hover:bg-gray-700 rounded text-base mt-4 md:mt-0",
        "Button"
        RightArrowIcon {}
    }
```

This is the common way in web development, it's very powerful, but not providing any checks, typos in the classes will be hard to notice.

## Basic Usage

### Define class constant with macro

To create constants for your custom css, or creating a crate for existing frameworks, constants are created for compile time check.

Since css use `-` as separator, macros are provided to generate rust const for them by replacing `-` to `_`

These macros are accessable in `dioxus_class::ext::*`;

```rust
constant!(btn success);
constant!(btn warning);
```

Will be expanded to:

```rust
pub const btn_success: &'static str = "btn-success";
pub const btn_warning: &'static str = "btn-warning";
```

### Define style for components

Ideally, most views can be organized as dioxus components, then can define style for each of them alongside the rendering logic.

This sample is based on dioxus-tailwindcss crate, extra build steps might be needed, a full sample will be provided there.

```rust
mod style {
    use dioxus_tailwindcss::prelude::*;
    use lazy_static::lazy_static;

    style!{
        header: lass!(text_gray_400 bg_gray_900 body_font),
        button: class!(inline_flex items_center bg_gray_800 border_0 py_1 px_3 focus(outline_none) hover(bg_gray_700) rounded text_base mt_4 md(mt_0)),
        // more class definition.
    }

    lazy_static! {
        pub static ref STYLE: Style = Style::default();
    }
}
use style::STYLE;

// then you can use these class constants in rendering logic, e.g.
//
// class: &STYLE.header,
```

### Add class together

It's common to add extra value to class based on data, Class can be added with &str, String, Vec, Array, Option types, to make this more easy to write.

```
    class: completed.clone() + editing,
```