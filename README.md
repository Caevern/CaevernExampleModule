# How to compile?
First make sure you have the wasm target installed:
```
rustup target add wasm32-unknown-unknown
```
Then, compile the project:
```
cargo build --release --target wasm32-unknown-unknown
```
Now you can find the compiled wasm module in `target/wasm32-unknown-unknown/release/caevern_example_module.wasm`.
