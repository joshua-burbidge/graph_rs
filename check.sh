cargo build
cargo build --target=wasm32-unknown-unknown
wasm-bindgen ./target/wasm32-unknown-unknown/debug/graph.wasm --out-dir wasm/generated --target web
