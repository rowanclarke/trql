import init, * as wasm from "./dist/site.js";

await init();

const tree = document.getElementById('tree');
const query = document.getElementById('query');
const output = document.getElementById('output');

function update() {
  output.value = wasm.execute(tree.value, query.value);
}

tree.addEventListener('input', update);
query.addEventListener('input', update);
