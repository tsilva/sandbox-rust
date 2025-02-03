use wasmer::{Store, Module, Instance, imports, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a Wasmer store (Required in Wasmer 5.0+)
    let mut store = Store::default();

    // Load the WebAssembly module
    let wasm_bytes = std::fs::read("simple.wasm")?;
    let module = Module::new(&store, &wasm_bytes)?;

    // Create an empty import object
    let import_object = imports! {};

    // Instantiate the module (Pass store as the first argument)
    let instance = Instance::new(&mut store, &module, &import_object)?;

    // Get the exported function
    let add = instance.exports.get_function("add")?;

    // Call the function (Pass store as the first argument)
    let result = add.call(&mut store, &[Value::I32(5), Value::I32(10)])?;

    // Print the result
    if let Value::I32(sum) = result[0] {
        println!("Result: {}", sum);
    }

    Ok(())
}
