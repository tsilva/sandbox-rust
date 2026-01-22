<div align="center">
  <img src="logo.png" alt="sandbox-rust" width="256"/>

  # sandbox-rust

  [![Rust](https://img.shields.io/badge/Rust-1.70+-orange?logo=rust)](https://www.rust-lang.org/)
  [![WebAssembly](https://img.shields.io/badge/WebAssembly-Enabled-654FF0?logo=webassembly)](https://webassembly.org/)
  [![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

  **A hands-on Rust learning sandbox featuring 17 examples and WebAssembly integration**

</div>

## Overview

A practical Rust learning environment with three distinct projects:

- **Examples** - 17 progressive Rust examples covering types, structures, enums, variables, and more
- **wasm-example** - Compile Rust to WebAssembly using `wasm-bindgen`
- **wasmer-example** - Load and execute WASM modules with the Wasmer runtime

## Project Structure

```
sandbox-rust/
├── examples/           # 17 Rust learning examples
│   └── src/
│       ├── main.rs
│       ├── 001-hello.rs
│       ├── 002-types.rs
│       └── ...
├── wasm-example/       # Rust → WebAssembly library
│   └── src/lib.rs
├── wasmer-example/     # WASM runtime executor
│   ├── src/main.rs
│   └── simple.wat
└── .devcontainer/      # Dev container setup
```

> **Note:** Each subdirectory is an independent Cargo project. There is no root-level `Cargo.toml`.

## Quick Start

### Run Examples

```bash
cd examples
cargo run
```

### Compile Rust to WebAssembly

```bash
cd wasm-example
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
```

### Execute WASM with Wasmer

```bash
cd wasmer-example
wat2wasm simple.wat -o simple.wasm
cargo run
```

## Learning Examples

| # | File | Topic |
|---|------|-------|
| 001 | `hello.rs` | Hello World |
| 002 | `types.rs` | Primitive types |
| 003 | `literals-operators.rs` | Literals & operators |
| 004 | `tuples.rs` | Tuples |
| 005 | `arrays-slices.rs` | Arrays & slices |
| 006 | `structures.rs` | Structs |
| 007 | `enums.rs` | Enums |
| 008 | `use.rs` | Use keyword |
| 009 | `enums-c.rs` | C-like enums |
| 010 | `linked-list.rs` | Linked lists |
| 011 | `constants.rs` | Constants |
| 012 | `variable-bindings.rs` | Variable bindings |
| 013 | `mutability.rs` | Mutability |
| 014 | `variable-scoping.rs` | Scoping |
| 015 | `variable-shadowing.rs` | Shadowing |
| 016 | `variable-declare-first.rs` | Declare first |
| 017 | `variable-freezing.rs` | Freezing |

## WebAssembly Projects

### wasm-example

A library crate that compiles Rust functions to WebAssembly using `wasm-bindgen`:

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

**Build:**
```bash
cd wasm-example
cargo build --target wasm32-unknown-unknown --release
# Output: target/wasm32-unknown-unknown/release/wasm_example.wasm
```

**Test:**
```bash
cargo test
```

### wasmer-example

An executable that loads and runs WASM modules using Wasmer 5.0+:

```rust
use wasmer::{Store, Module, Instance, imports, Value};

let mut store = Store::default();
let wasm_bytes = std::fs::read("simple.wasm")?;
let module = Module::new(&store, &wasm_bytes)?;
let instance = Instance::new(&mut store, &module, &imports! {})?;

let add = instance.exports.get_function("add")?;
let result = add.call(&mut store, &[Value::I32(5), Value::I32(10)])?;
```

**Run:**
```bash
cd wasmer-example
wat2wasm simple.wat -o simple.wasm
cargo run
# Output: Result: 15
```

## Dependencies

| Project | Crate | Version | Purpose |
|---------|-------|---------|---------|
| examples | `rand` | 0.9.0 | Random number generation |
| wasm-example | `wasm-bindgen` | 0.2 | Rust/JavaScript interop |
| wasmer-example | `wasmer` | 5.0.4 | WebAssembly runtime |

## Development Environment

The `.devcontainer/` includes a complete setup:

- Rust with Cargo
- Wasmer runtime
- WABT tools (`wat2wasm`)
- wasm-pack
- Node.js 18.16.0 via NVM

## License

MIT
