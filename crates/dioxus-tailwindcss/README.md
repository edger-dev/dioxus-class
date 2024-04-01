# dioxus-tailwindcss

- [Tailwind CSS](https://tailwindcss.com)
- v3.4.3

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
| dark_mode.rs | [link](https://tailwindcss.com/docs/dark-mode) | use `dark_` instead of `dark` | 
| layout.rs | [link](https://tailwindcss.com/docs/aspect-ratio) Layout | use `static_` instead of `static` | 
| flex.rs | [link](https://tailwindcss.com/docs/flex-basics) Flexbox & Grid | | 
| spacing.rs | [link](https://tailwindcss.com/docs/padding) Spacing | | 

| sizing.rs | [link](https://tailwindcss.com/docs/width) Sizing | | 
| typography.rs | [link](https://tailwindcss.com/docs/font-family) Typography | | 
| backgrounds.rs | [link](https://tailwindcss.com/docs/background-attachment) Backgrounds | | 
| borders.rs | [link](https://tailwindcss.com/docs/border-radius) Borders | | 
| effects.rs | [link](https://tailwindcss.com/docs/box-shadow) Effects | | 
| filters.rs | [link](https://tailwindcss.com/docs/blur) Filters | | 
| tables.rs | [link](https://tailwindcss.com/docs/border-collapse) Tables | | 
| tansitions.rs | [link](https://tailwindcss.com/docs/transition-property) Transitions & Animation | | 
| transforms.rs | [link](https://tailwindcss.com/docs/scale) Transforms | | 
| interactivity.rs | [link](https://tailwindcss.com/docs/accent-color) Interactivity | | 
| svg.rs | [link](https://tailwindcss.com/docs/fill) SVG | | 
| accessibility.rs | [link](https://tailwindcss.com/docs/screen-readers) Accessibility | | 


## Build All Used CSS Classes 

Since tailwindcss need to get all used values, when using `class!`, the default build process is not working.

Check [BUILD.md](https://github.com/edger-dev/dioxus-class/tree/main/BUILD.md) for how to handle this process.