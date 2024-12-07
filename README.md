# graph_rs

## Usage

To graph some demo equations:
```
cargo run -- --demo
```


To enter custom equations:
```
cargo run
```
- exponents must be integers

![graph_rs_2](https://github.com/user-attachments/assets/7859fb34-b5bb-42c8-8ea6-66cfb95e900d)


-----------------------

#### TODO
- reverse order - create equation from graph
- use bezier curves
- highlight points hovered
- don't always graph full x domain if it's off-screen in the y


------------

wasm

cargo build --target=wasm32-unknown-unknown
wasm-bindgen ./target/wasm32-unknown-unknown/debug/graph.wasm --out-dir wasm/generated --target web
cd wasm
python3 -m http.server


currently, the compilation and wasm generation works but the site is empty
maybe something with the cargo cli args?
