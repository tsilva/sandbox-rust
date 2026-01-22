# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust learning sandbox with three distinct projects:
1. **Main project** (`src/`) - Simple random number generator using the `rand` crate
2. **Examples** (`examples/`) - Collection of 17 Rust learning examples (types, structures, enums, variables, etc.)
3. **WebAssembly projects** - Two WASM-related projects demonstrating compilation and execution

## Project Structure

The repository contains three separate Cargo projects:
- `examples/` - Standalone examples project with `main.rs` and numbered example files
- `wasm-example/` - Library crate for compiling Rust to WebAssembly using `wasm-bindgen`
- `wasmer-example/` - Executable that loads and runs WASM modules using Wasmer runtime

Note: There is no root-level `Cargo.toml` - each subdirectory is its own independent project.

## Build and Run Commands

### Main Project
```bash
cargo build              # Build the main project
cargo run                # Run random number generator
```

### Examples Project
```bash
cd examples
cargo build
cargo run                # Runs examples/src/main.rs
```

### WebAssembly Workflow

**Compiling WAT to WASM:**
```bash
cd wasmer-example
wat2wasm simple.wat -o simple.wasm
```

**Running WASM directly with Wasmer:**
```bash
wasmer simple.wasm
```

**Running WASM through Rust (wasmer-example):**
```bash
cd wasmer-example
cargo run                # Loads simple.wasm and calls exported functions
```

**Compiling Rust to WASM:**
```bash
# Add wasm32 target (one-time setup)
rustup target add wasm32-unknown-unknown

# Compile wasm-example to WASM
cd wasm-example
cargo build --target wasm32-unknown-unknown --release
# Output: target/wasm32-unknown-unknown/release/wasm_example.wasm
```

## Testing

The wasm-example library includes unit tests:
```bash
cd wasm-example
cargo test
```

## Development Environment

The `.devcontainer/Dockerfile` sets up a complete development environment including:
- Rust with cargo
- Wasmer runtime (WebAssembly executor)
- WABT tools (wat2wasm for WAT→WASM compilation)
- wasm-pack (Rust→WASM tooling)
- Node.js 18.16.0 via NVM

All tools are installed for the `devuser` user with appropriate PATH configuration.

## Key Dependencies

- `rand = "0.9.0"` - Random number generation (main project and examples)
- `wasmer = "5.0.4"` - WebAssembly runtime (wasmer-example)
- `wasm-bindgen = "0.2"` - JavaScript/Rust interop for WASM (wasm-example)

## Architecture Notes

**wasmer-example execution flow:**
1. Creates a Wasmer `Store` (required in v5.0+)
2. Loads WASM bytecode from `simple.wasm` file
3. Creates a `Module` from bytecode
4. Instantiates the module with an empty import object
5. Gets exported functions and calls them with store reference

**wasm-example library:**
- Configured as both `cdylib` (for WASM compilation) and `rlib` (for Rust testing)
- Uses `#[wasm_bindgen]` macro to export functions for JavaScript interop
- Contains both WASM-exposed functions and regular Rust functions

## Important Instructions

- README.md must be kept up to date with any significant project changes
