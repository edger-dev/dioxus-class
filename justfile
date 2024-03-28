build:
    cargo build

publish-all:
    cd crates/dioxus-class && cargo publish
    cd crates/dioxus-tailwindcss && cargo publish
    cd crates/dioxus-daisyui && cargo publish

build-doc:
    cargo doc --no-deps

serve-doc:
    simple-http-server -p 8001 --index --nocache target/doc

install-dioxus-cli:
    cargo install dioxus-cli


