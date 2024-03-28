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

### Add class together

It's common to add extra value to class based on data, Class can be added with &str, String, Vec, Array, Option types, to make this more easy to write.

```
    class: completed.clone() + editing,
```
