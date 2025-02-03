import { fileURLToPath } from 'url';
import { dirname } from 'path';
import fs from 'fs';
import init, { greet } from './pkg/wasm_example.js';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const wasmPath = `${__dirname}/pkg/wasm_example_bg.wasm`;
const wasmBuffer = fs.readFileSync(wasmPath);

async function run() {
    try {
        await init(wasmBuffer);
        console.log(greet("WebAssembly"));
    } catch (err) {
        console.error('WASM initialization error:', err);
        throw err;
    }
}

run().catch(console.error);
