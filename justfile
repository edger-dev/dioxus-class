build:
    cargo build

publish-all:
    cd crates/dioxus-class && cargo publish
    cd crates/dioxus-tailwindcss && cargo publish
    cd crates/dioxus-daisyui && cargo publish

install-dioxus-cli:
    cargo install dioxus-cli

install-dioxus-cli_main:
    cargo install --git https://github.com/DioxusLabs/dioxus dioxus-cli
