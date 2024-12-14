use pest::Parser;

use crate::{
    command::Command,
    parser::{QueryParser, Rule},
    query::{execute, FromNodes, Operation, Query, Series},
    tree::{Descendants, DynNodes, Node, Tree},
};

use super::parser::to_queries;
use std::rc::Rc;

mod tree;
use tree::*;

// #[derive(Debug, FromNodes)]
// struct Item {
//     id: String,
//     content: String,
// }

// fn test_tree() -> TestTree {
//     TestTree::new(Rc::new(vec![
//         TestNodeData::new("item", Some("item a"), 5),
//         TestNodeData::new("id", Some("a"), 0),
//         TestNodeData::new("content", Some("A"), 0),
//         TestNodeData::new("item", Some("item b"), 2),
//         TestNodeData::new("id", Some("b"), 0),
//         TestNodeData::new("content", Some("B"), 0),
//         TestNodeData::new("item", Some("item c"), 2),
//         TestNodeData::new("id", Some("c"), 0),
//         TestNodeData::new("content", Some("C"), 0),
//         TestNodeData::new("item", Some("item d"), 2),
//         TestNodeData::new("id", Some("d"), 0),
//         TestNodeData::new("content", Some("D"), 0),
//     ]))
// }

// #[test]
// fn tree() {
//     let tree = test_tree();

//     let queries = to_queries(
//         QueryParser::parse(
//             Rule::queries,
//             "…item 1
//   id = id
//   content = content",
//         )
//         .unwrap(),
//     );

//     let result: Vec<Item> = execute(queries, tree);
//     println!(">> {:?}", result);
// }
