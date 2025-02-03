# sandbox-rust

A simple Rust program that generates random numbers using the `rand` crate.

## Requirements

- Rust (latest stable version)
- Cargo (comes with Rust)

## Setup

1. Clone the repository
2. Build the project:
```bash
cargo build
```

## Running

Execute the program with:
```bash
cargo run
```

The program will generate a random number between 1 and 100.

## Dependencies

- rand = "0.9.0" - For random number generation

## WebAssembly Example

### Compiling WAT to WASM
1. First, compile the WebAssembly text format (.wat) to binary format (.wasm):
```bash
wat2wasm simple.wat -o simple.wasm
```

### Running WebAssembly with Wasmer directly
You can run the WASM file directly using Wasmer:
```bash
wasmer simple.wasm
```

### Running WebAssembly with Rust Example
1. Navigate to the wasmer-example directory:
```bash
cd wasmer-example
```

2. Make sure simple.wasm is in the current directory

3. Run the Rust example:
```bash
cargo run
```

This will execute the Rust program that loads the WebAssembly module and calls the `add` function with parameters 5 and 10, displaying the result.

## Rust to WebAssembly Example

### Compiling Rust to WASM
1. Add wasm32 target:
```bash
rustup target add wasm32-unknown-unknown
```

2. Compile the library to WASM:
```bash
cd wasmer-example
cargo build --target wasm32-unknown-unknown --release
```

This will create a WASM file at `target/wasm32-unknown-unknown/release/wasmer_example.wasm`

3. Run the example:
```bash
cargo run
```

This will load the compiled WASM module and run the multiply function with parameters 7 and 6.
