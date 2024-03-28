# Build Process

For CSS frameworks such as tailwindcss to work, an extra build process is needed, which usually use regular expression to find all source codes for possible classes. Since we are using rust constants and functions instead of plain strings, this is not working at all.

It's quite tricky to get it work in regular expression in source code level, so I stucked for a better solution, previous I used dioxus-ssr to generate a few html pages, then use that as input to tailwindcss, which works, but quite messy, also might missed some classes if the related page wasn't generated.

Later I found a much nicer solutions by [procout](https://github.com/plasticartsshow/procout), which add file generation in `proc_macro!`, so I borrow the idea, so far it seems works pretty good.

Check [Emoji Browser](https://github.com/edger-dev/dioxus-class/tree/main/demos/emoji-browser) for a complete sample.

## How It Works

A special feature `build-classes` is added, when it's activated, all `class!` calls will be written to `classes.rs`, which contains all generated rust codes.

We will run `cargo test --features "build-classes"` to generate that file, then we will include the generated file in `build.rs` to create the html file with all CSS classes, then that file can be used as input for tailwindcss.

## Build Steps

### Generate `classes.rs`

I'm using [just](https://github.com/casey/just) to run this step:

```just
build-classes:
    cargo test --features "web build-classes"
    echo "]" >> ../../classes.rs
    mv -v ../../classes.rs .
```

### Sample `build.rs`

```rust
use std::env;
use std::path::Path;

use dioxus_daisyui::build::generate_classes;
use dioxus_daisyui::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=styles.rs");
    // let out_dir = env::var_os("OUT_DIR").unwrap();
    let current_dir = env::current_dir()?;
    let css_dir = Path::new(&current_dir).join("css");

    let classes_path = Path::new(&css_dir).join("classes.html");
    let classes = include!("classes.rs");
    generate_classes(classes_path, classes)?;
    
    dioxus_daisyui::build::tailwindcss(css_dir, "tailwind.input.css", "../public/css/tailwind.css")
}
```

## Caveats

### Missing `]` in classes.rs

For the `classes.rs` to be used by `include!()`, it must contain a single expression, I don't know how to write the final `]`, so current solution is to add it in build script.

### Initial content for `classes.rs`

For `build.rs` to work, a proper `classes.rs` is needed, before step 1 first done, need to either comment out the related codes in build.rs or rename `build.rs` temporary, you can also manually create a `classes.rs` with content as `vec![]`

### `classes.rs` Generated at Workspace Root Path

I'd like it to be generated under project path, but don't know how to do that yet. 