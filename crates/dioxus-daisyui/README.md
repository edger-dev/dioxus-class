# dioxus-daisyUI

- [Daisy UI](https://daisyui.com)
- v4.9.0

## Documents for Updating

| Code | Documentation | Notes |
| ---- | ------------- | ----- |
| actions.rs | [link](https://daisyui.com/components/button/) Actions |
| display.rs | [link](https://daisyui.com/components/accordion/) Data display |
| navigation.rs | [link](https://daisyui.com/components/breadcrumbs/) Navigation | use `active_ disabled_ focus_` instead of `active disabled focus`, the names been taken by tailwindcss as modifiers |
| feedback.rs | [link](https://daisyui.com/components/alert/) Feedback |
| input.rs | [link](https://daisyui.com/components/checkbox/) Data input |
| layout.rs | [link](https://daisyui.com/components/artboard/) Layout |
| mockup.rs | [link](https://daisyui.com/components/mockup-browser/) Mockup|

## Build All Used CSS Classes 

Since tailwindcss need to get all used values, when using `class!`, the default build process is not working.

Check [BUILD.md](https://github.com/edger-dev/dioxus-class/tree/main/BUILD.md) for how to handle this process.
