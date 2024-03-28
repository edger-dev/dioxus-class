# dioxus-class-macro

`class!` proc_macro provided to define class clearly

## Example
```rust
   rsx! {
       div {
           class: class!(card card_compact w_64 h_64 bg_base_300 shadow_xl text_center hover(bg_base_200) hover(scale_105)),
           div {
               class: class!(text_8xl py_10),
               "{value}",
           },
           div {
               class: class!(card_body text_center items_center),
               div {
                   class: class!(card_title text_sm text_base_content),
                   "{alias}",
               }
           }
       }
   }
```

The elements such as `card_compact` are just normal rust expressions, they are all checked by compiler,
and when defined as constants, can provide auto-complete in editor. The only requirement is that they 
can be converted to String with String::from()

