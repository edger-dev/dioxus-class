# dioxus-tailwindcss

- https://tailwindcss.com
- v3.2.7

## Tricks

Due to rust's rule on constant names, there are some difference from tailwindcss.

| tailwindcss | dioxus-tailwindcss |
| ----------- | ------------------ |
| `static` | `static_` |
| `2xl:` | `xxl()` and `_2xl()` |
| fraction e.g. `-1/2` | `_1__2` |
| `.5` | `_half` |

## Documents for Updating

| Code | Documentation | Notes |
| ---- | ------------- | ----- |
| modifier.rs | [link](https://tailwindcss.com/docs/hover-focus-and-other-states) | check the #pseudo-class-reference part for complete list |
| responsive.rs | [link](https://tailwindcss.com/docs/responsive-design) | 2xl is not a valid rust function name, define both `xxl` and `_2xl` for it |
| layout.rs | [link](https://tailwindcss.com/docs/position) | use `static_` instead of `static` | 

TODO: add link and notes for all modules

## Build All Used CSS Classes 

Since tailwindcss need to get all used values, when using `class!`, the default build process is not working.

Check [BUILD.md](https://github.com/edger-dev/dioxus-class/tree/main/BUILD.md) for how to handle this process.