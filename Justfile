[working-directory: "rust"]
build-rust-wasm:
    cargo +nightly build -Zbuild-std --target wasm32-unknown-emscripten

[working-directory: "rust"]
build-rust:
    cargo build

[working-directory: "rust"]
build-rust-wasm-release:
    cargo +nightly build -Zbuild-std --release --target wasm32-unknown-emscripten

[working-directory: "rust"]
build-rust-release:
    cargo build --release

