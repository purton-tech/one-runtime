# Islands

Client-side interactivity that cannot be handled with server rendering alone.

## Structure

- `src/lib.rs` WASM entrypoints
- `src/*.rs` small behavior-focused browser modules

## Workflow

- Keep islands logic small and behavior-specific.
- Use this crate only for client-side interactivity that needs browser APIs or event handling after render.
- WASM output is deployed through `crates/one-runtime-assets/dist`.

## Build Commands

Reference only unless explicitly required:

- `cargo build -p one-runtime-islands --target wasm32-unknown-unknown`
- `wasm-bindgen target/wasm32-unknown-unknown/release/one_runtime_islands.wasm --target web --out-dir crates/one-runtime-assets/dist`
