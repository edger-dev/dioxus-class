build:
    cargo build

publish-all:
    cd crates/dioxus-class && cargo publish
    cd crates/dioxus-tailwindcss && cargo publish
    cd crates/dioxus-daisyui && cargo publish

install-dioxus-cli:
    cargo install dioxus-cli@0.5.0-alpha.2
