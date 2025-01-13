import init, * as wasm from "./dist/site.js";

await init();

const examples = [
  {
    subheading: "Get all \"node\" nodes of the tree.",
    tree: "node: ab\n\
  name: a\n\
  value: b\n\
  node: cd\n\
    name: c\n\
    value: d\n\
node: ef\n\
  name: e\n\
  value: f",
    query: "…node",
    exercises: [
      "Get only the nodes at the base of the tree.",
      "Get all nodes of the subtree of the first node of the tree.",
      "Get all \"name\" nodes of all \"node\" nodes of the tree."
    ]
  },
  {
    subheading: "Get the name and value of all \"node\" nodes of the tree.",
    tree: "node: ab\n\
  name: a\n\
  value: b\n\
  node: cd\n\
    name: c\n\
    value: d\n\
node: ef\n\
  name: e\n\
  value: f",
    query: "…node\n\
  name = name\n\
  value = value",
    exercises: [
      "Get the name and value of only the nodes that have at least one child node."
    ]
  }
];


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
  theme: "trql",
  readOnly: true
});
const output_textarea = output_editor.getWrapperElement();
output_textarea.id = "output-editor";

function update() {
  output_editor.setValue(wasm.execute(tree_editor.getValue(), query_editor.getValue()));
  
  console.log("Hi");
}

const example_title = document.getElementById("example-title");
const example_subheading = document.getElementById("example-subheading");
const example_exercises = document.getElementById("example-exercises");
const prev_example = document.getElementById("prev-example");
const next_example = document.getElementById("next-example");
const examples_len = examples.length;
let example_index = 0;

function update_example() {
  let example = examples[example_index];
  example_title.textContent = "Example " + (example_index + 1);
  example_subheading.textContent = example.subheading;
  example_exercises.textContent = "";
  example.exercises.forEach(item => {
    let li = document.createElement("li");
    li.textContent = item;
    example_exercises.appendChild(li);
  })
  tree_editor.setValue(example.tree);
  query_editor.setValue(example.query);
}

prev_example.addEventListener('click', () => {
  example_index = (example_index + examples_len - 1) % examples_len;
  update_example();
});

next_example.addEventListener('click', () => {
  example_index = (example_index + 1) % examples_len;
  update_example();
});

update_example();

