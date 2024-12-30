import init, * as wasm from "./dist/site.js";

await init();
wasm.greet();

