# [Dioxus Class](https://github.com/edger-dev/dioxus-class)

When using dioxus for GUI development, by default plain strings are used for CSS class, which is not ideal IMO, there is no compile time validation, and also the auto completion is only text based, and it's not easy to reuse class constants.

## [dioxus-class](https://github.com/edger-dev/dioxus-class/tree/main/crates/dioxus-class)

<div class="crate">
  <!-- Crates version -->
  <a href="https://crates.io/crates/dioxus-class">
    <img src="https://img.shields.io/crates/v/dioxus-class.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/dioxus-class">
    <img src="https://img.shields.io/crates/d/dioxus-class.svg?style=flat-square"
      alt="Download" />
  </a>
</div>

Class type been provided, which is just a wrapper struct around `Vec<String>`, some basic `From<>` and `Add<>` implementation make it easier to be used.

`constant!` macro for easier definition for css values, e.g.

```rust
constant!(table column group);
```

will be transformed to:

```rust
pub const table_column_group: &'static str = "table-column-group";
```

`class!` macro provided for easier definition using string constants, e.g.

```rust
cx.render(rsx!{
    div {
        class: class!(card_body text_center items_center hover(scale_105)),
        div {
            class: class!(card_title text_sm text_base_content),
            cx.props.alias
        }
    }
})
```

## [dioxus-tailwindcss](https://github.com/edger-dev/dioxus-class/tree/main/crates/dioxus-tailwindcss)

<div class="crate">
  <!-- Crates version -->
  <a href="https://crates.io/crates/dioxus-tailwindcss">
    <img src="https://img.shields.io/crates/v/dioxus-tailwindcss.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/dioxus-tailwindcss">
    <img src="https://img.shields.io/crates/d/dioxus-tailwindcss.svg?style=flat-square"
      alt="Download" />
  </a>
</div>

Defined constants and modifiers for [tailwindcss](https://tailwindcss.com) v3.2.7

## [dioxus-daisyui](https://github.com/edger-dev/dioxus-class/tree/main/crates/dioxus-daisyui)

<div class="crate">
  <!-- Crates version -->
  <a href="https://crates.io/crates/dioxus-daisyui">
    <img src="https://img.shields.io/crates/v/dioxus-daisyui.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/dioxus-daisyui">
    <img src="https://img.shields.io/crates/d/dioxus-daisyui.svg?style=flat-square"
      alt="Download" />
  </a>
</div>

Defined constants and modifiers for [daisyui](https://daisyui.com) v2.50.2

## [Emoji Browser](https://github.com/edger-dev/dioxus-class/tree/main/demos/emoji-browser)

<a href="https://www.edger.dev/emoji/browser" target="_blank">Open web app in new tab</a>

Search emoji by shortcode, built as demo project.
