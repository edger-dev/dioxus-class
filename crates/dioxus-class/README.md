# dioxus-class

Most dioxus samples are using plain strings for classes, e.g.

```rust
    button {
        class: "inline-flex border-0 py-1 px-3 focus:outline-none hover:bg-gray-700",
        "Button"
        RightArrowIcon {}
    }
```

This is the common way in web development, it's very powerful, but not providing any checks, typos in the classes will be hard to notice.

## How it Works

To support a certain CSS framework, Simple CSS values are defined as string constants, modifiers like `hover:` are defined as functions, then you can write the previous example as following:

```rust
    button {
        class: class!(inline_flex border_0 py_1 px_3 focus(outline_none) hover(bg_gray_700)),
        "Button"
        RightArrowIcon {}
    }
```

It looks very clean, and show the original values clearly, most importantly, the compiler will catch any typo now.


## How to Create CSS framework

check [dioxus-tailwindcss](https://github.com/edger-dev/dioxus-class/tree/main/crates/dioxus-tailwindcss) for more details.

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

### Define sets of constants

There are many similar value groups, which is not easy to define manually, simple macro rules can be created to make this easier.

```rust
#[macro_export]
macro_rules! colors {
    ( $( $prefix:ident )* ) => {
        constant!($( $prefix )* inherit);
        constant!($( $prefix )* current);
        constant!($( $prefix )* transparent);
        constant!($( $prefix )* black);
        constant!($( $prefix )* white);

        constant!($( $prefix )* slate 50);
        constant!($( $prefix )* slate 100);
        constant!($( $prefix )* slate 200);
        constant!($( $prefix )* slate 300);
        constant!($( $prefix )* slate 400);
        constant!($( $prefix )* slate 500);
        constant!($( $prefix )* slate 600);
        constant!($( $prefix )* slate 700);
        constant!($( $prefix )* slate 800);
        constant!($( $prefix )* slate 900);

        constant!($( $prefix )* gray 50);
        constant!($( $prefix )* gray 100);
        constant!($( $prefix )* gray 200);
        constant!($( $prefix )* gray 300);
        constant!($( $prefix )* gray 400);
        constant!($( $prefix )* gray 500);
        constant!($( $prefix )* gray 600);
        constant!($( $prefix )* gray 700);
        constant!($( $prefix )* gray 800);
        constant!($( $prefix )* gray 900);
        ...
    }
}
```

```rust
// https://tailwindcss.com/docs/border-color
colors!(border);
colors!(border x);
colors!(border y);
colors!(border t);
colors!(border r);
colors!(border b);
colors!(border l);
```

## Build All Used CSS Classes 

Since tailwindcss need to get all used values, when using `class!`, the default build process is not working.

Check [BUILD.md](https://github.com/edger-dev/dioxus-class/tree/main/BUILD.md) for how to handle this process.
