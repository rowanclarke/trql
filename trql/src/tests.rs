use trql_derive::QueryResult;

use crate::{
    query::{execute, Queries, QueryResult},
    tree::{DynNodes, Node, Tree},
};

use std::{iter::once, rc::Rc};

mod tree;
use tree::*;

#[derive(PartialEq, Eq, Debug, QueryResult)]
struct Item {
    id: String,
    content: String,
}

fn test_tree() -> TestTree {
    TestTree::new(Rc::new(vec![
        TestNodeData::new("item", Some("item a"), 5),
        TestNodeData::new("id", Some("a"), 0),
        TestNodeData::new("content", Some("A"), 0),
        TestNodeData::new("item", Some("item b"), 2),
        TestNodeData::new("id", Some("b"), 0),
        TestNodeData::new("content", Some("B"), 0),
        TestNodeData::new("item", Some("item c"), 2),
        TestNodeData::new("id", Some("c"), 0),
        TestNodeData::new("content", Some("C"), 0),
        TestNodeData::new("item", Some("item d"), 2),
        TestNodeData::new("id", Some("d"), 0),
        TestNodeData::new("content", Some("D"), 0),
    ]))
}

#[test]
fn tree() {
    let tree = test_tree();

    let result: Vec<Item> = execute(
        "…item 1
  id = id
  content = content",
        tree,
    );
    assert_eq!(
        result,
        vec![Item {
            id: "a".into(),
            content: "A".into()
        }]
    )
}
