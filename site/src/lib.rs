mod object;
mod tree;

use object::Object;
use pest::Parser;
use tree::{to_tree, Rule, TreeParser};
use trql::query;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn execute(tree: &str, query: &str) -> String {
    let tree = to_tree(TreeParser::parse(Rule::nodes, tree).unwrap());
    serde_yml::to_string(&query::execute::<_, Object>(query, tree)).unwrap()
}
