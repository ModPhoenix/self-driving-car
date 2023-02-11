## Getting started

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup default nightly` - setup nightly as default, or you can use rust-toolchain file later on
3. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly

## Running your project

```shell
# Install via cargo.
cargo install --locked trunk
# Until wasm-bindgen has pre-built binaries for Apple M1, M1 users will
# need to install wasm-bindgen manually.
cargo install --locked wasm-bindgen-cli
```

`trunk serve`
