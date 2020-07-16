# Simple form example

To run:

Install the following if desired:
```bash
cargo install wasm-pack
cargo +nightly install miniserve
```

Compile and run:
```bash
wasm-pack build --target web --out-name wasm --out-dir ./static
miniserve ./static --index index.html
```