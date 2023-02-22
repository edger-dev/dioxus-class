build:
    cargo build

publish-all:
    cd crates/dioxus-class && cargo publish
    cd crates/dioxus-daisyui && cargo publish
