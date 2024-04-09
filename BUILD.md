# Build Process

For CSS frameworks such as tailwindcss to work, an extra build process is needed, which usually use regular expression to find all source codes for possible classes. Since we are using rust constants and functions instead of plain strings, this is not working at all.

It's quite tricky to get it work in regular expression in source code level, so I stucked for a better solution, previous I used dioxus-ssr to generate a few html pages, then use that as input to tailwindcss, which works, but quite messy, also might missed some classes if the related page wasn't generated.

Later I found a much nicer solutions by [procout](https://github.com/plasticartsshow/procout), which add file generation in `proc_macro!`, so I borrow the idea, so far it seems works pretty good.

Check [Emoji Browser](https://github.com/edger-dev/dioxus-class/tree/main/demos/emoji-browser) for a complete sample.

## How It Works

A special feature `build-classes` is added, when it's activated, all `class!` calls will be written to `classes.rs`, which contains all generated rust codes.

We will run `cargo test --features "build-classes"` to generate that file, then we will include the generated file in `build.rs` to create the html file with all CSS classes, then that file can be used as input for tailwindcss.

## Build Steps

I'm using [just](https://github.com/casey/just) to run these steps, also provided sample for watching src folder and rebuild the classes with `cargo watch`

```just
build-classes:
    DIOXUS_CLASS_BUILD_PATH="$PWD/css/classes.rs" cargo test --features "web build-classes"
    cargo build
    cd css && cargo build

clear-buffer:
    echo -e -n "\\0033c" && tmux clear-history

watch-classes:
    cargo watch -w src/ -s "just clear-buffer && cargo rustc --lib -- -Awarnings && just build-classes"
```


### Generate `classes.rs`

`DIOXUS_CLASS_BUILD_PATH="$PWD/css/classes.rs" cargo test --features "web build-classes"`

By running `cargo test` with `build-classes` feature, classes.rs will be created at given position.

Note that by default, cargo will also run doctest, so the file will be generated twice, adding this to `Cargo.toml` can bypass the doctest running

```
[lib]
doctest = false
```

### Make sure `classes.rs` is having proper format

[build.rs](https://github.com/edger-dev/dioxus-class/tree/main/demos/emoji-browser/build.rs)

`check_classes` will check the given file, if it's not there, create an empty one with `vec![]` as content, of checking whether it ends with `]`, if not, adding one. 

This step is needed, since I can't find a way to execute cleanup logic in the classes generation, for that file to be used later, it must be a valid `vec<Class>`. 

```rust
use std::env;
use std::path::Path;
use dioxus_daisyui::build::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = env::current_dir()?;
    let css_dir = Path::new(&current_dir).join("css");
    let classes_path = Path::new(&css_dir).join("classes.rs");
    check_classes(classes_path)
}
```

### Generate `classes.html` and build with tailwindcss

[A simple cargo project created for this step](https://github.com/edger-dev/dioxus-class/tree/main/demos/emoji-browser/css)

[build.rs](https://github.com/edger-dev/dioxus-class/tree/main/demos/emoji-browser/css/build.rs)

```
use std::env;
use std::path::Path;

use dioxus_daisyui::build::*;
use dioxus_daisyui::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=classes.rs");
    // let out_dir = env::var_os("OUT_DIR").unwrap();
    let current_dir = env::current_dir()?;

    let classes_path = Path::new(&current_dir).join("classes.html");
    let classes = include!("classes.rs");
    generate_classes(classes_path, classes)?;
    
    npx_tailwindcss(current_dir, "tailwind.input.css", "../assets/css/tailwind.css")
}

```