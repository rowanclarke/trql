use pest::Parser;

use crate::{
    parser::{Operation, Query, QueryParser, Rule},
    tree::Node,
    Command,
};

use super::parser::to_queries;
use std::rc::Rc;

mod tree;
use tree::*;

#[test]
fn query() {
    let queries = to_queries(
        QueryParser::parse(
            Rule::queries,
            "…node
  id = id
  content = content",
        )
        .unwrap(),
    );
    println!("{:?}", queries);
}

fn test_tree() -> TestTree {
    TestTree::new(Rc::new(vec![
        TestNodeData::new("node", None, 5),
        TestNodeData::new("id", Some("a"), 0),
        TestNodeData::new("content", Some("A"), 0),
        TestNodeData::new("node", None, 2),
        TestNodeData::new("id", Some("b"), 0),
        TestNodeData::new("content", Some("B"), 0),
        TestNodeData::new("node", None, 2),
        TestNodeData::new("id", Some("c"), 0),
        TestNodeData::new("content", Some("C"), 0),
        TestNodeData::new("node", None, 2),
        TestNodeData::new("id", Some("d"), 0),
        TestNodeData::new("content", Some("D"), 0),
    ]))
}

#[test]
fn tree() {
    let tree = test_tree();

    let queries = to_queries(
        QueryParser::parse(
            Rule::queries,
            "…node
  id = id
  content = content",
        )
        .unwrap(),
    );
}

#[test]
fn children() {
    let tree = test_tree();

    println!(
        "{:?}",
        Operation::Children
            .execute::<TestTree, _>(tree)
            .map(|node| node.value().map(|s| s.to_string()))
            .collect::<Vec<_>>()
    );
}
