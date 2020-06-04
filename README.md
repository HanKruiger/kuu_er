# Kuu Er

A very minimalistic web app for generating QR codes and saving them as PNG images.

## How to build

Requirement: [`wasm-pack`](https://crates.io/crates/wasm-pack)

```
wasm-pack build --target web --out-dir dist/pkg
```

then serve the `dist` directory for local testing (e.g. `http dist`, see [`https`](https://crates.io/crates/https)).
