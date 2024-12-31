mod object;
mod tree;

use object::ObjectString;
use pest::Parser;
use tree::{to_tree, Rule, TreeParser};
use trql::query;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn execute(tree: &str, query: &str) -> String {
    let tree = to_tree(TreeParser::parse(Rule::nodes, tree).unwrap());
    jsonxf::pretty_print(&query::execute::<_, ObjectString>(query, tree).0).unwrap()
}
