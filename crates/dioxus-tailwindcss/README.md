# tailwindcss v3.4.1

- https://tailwindcss.com

## Update Process

Follow each page in tailwindcss documentation, checking what's changed

| Code | Documentation | Notes |
| ---- | ------------- | ----- |
| modifier.rs | https://tailwindcss.com/docs/hover-focus-and-other-states | check the #pseudo-class-reference part for complete list |
| responsive.rs | https://tailwindcss.com/docs/responsive-design | 2xl is not a valid rust function name, define both `xxl` and `_2xl` for it |