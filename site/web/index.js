import init, * as wasm from "./dist/site.js";

await init();

CodeMirror.defineSimpleMode("tree", {
  start: [{ regex: /[a-z]+:?/, token: "name", next: "value" }],
  value: [{ regex: /[a-z]+/, token: "value", next: "start" }]
});

CodeMirror.defineSimpleMode("trql", {
  start: [
    { regex: /[a-z]+(?= *=)/, token: "name" },
    { regex: /[a-z]+/, token: "token" },
    { regex: /[\-:0-9]+/, token: "range" },
    { regex: /[\.…,\(\)\[\]]/, token: "other" }
  ]
});


CodeMirror.defineSimpleMode("yaml", {
  start: [
    { regex: /[a-z]+:/, token: "name" },
    { regex: /-/, token: "other" },
    { regex: /[a-z]+/, token: "value" },
    { regex: /./, token: "other" }
  ]
});

const query_editor = CodeMirror.fromTextArea(document.getElementById("query-textarea"), {
  mode: "trql",
  theme: "trql"
});
query_editor.getWrapperElement().id = "query-editor";
query_editor.on("change", update);

const tree_editor = CodeMirror.fromTextArea(document.getElementById("tree-textarea"), {
  mode: "tree",
  theme: "trql"
});
tree_editor.getWrapperElement().id = "tree-editor";
tree_editor.on("change", update);

const output_editor = CodeMirror.fromTextArea(document.getElementById("output-textarea"), {
  mode: "yaml",
  theme: "trql"
});
const output_textarea = output_editor.getWrapperElement();
output_textarea.id = "output-editor";

function update() {
  output_editor.setValue(wasm.execute(tree_editor.getValue(), query_editor.getValue()));
}
