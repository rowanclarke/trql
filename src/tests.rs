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

#[test]
fn query() {
    let queries = to_queries(
        QueryParser::parse(
            Rule::queries,
            "…item
  id = id
  content = content",
        )
        .unwrap(),
    );
    println!("{:?}", queries);
}

#[derive(Debug)]
struct Item {
    id: String,
    content: String,
}

impl<T: Tree + 'static> FromNodes<T> for Item {
    fn from_nodes(queries: Vec<Query>, nodes: Box<dyn DynNodes<T>>) -> Self {
        let mut id: Option<String> = None;
        let mut content: Option<String> = None;
        for query in queries {
            match query.into_named() {
                Some((name, select, subqueries)) if name == "id" => {
                    id = Some(String::from_nodes(
                        subqueries,
                        select.execute::<T, _>(nodes.clone()),
                    ))
                }
                Some((name, select, subqueries)) if name == "content" => {
                    content = Some(String::from_nodes(
                        subqueries,
                        select.execute::<T, _>(nodes.clone()),
                    ))
                }
                _ => (),
            }
        }
        Self {
            id: id.unwrap(),
            content: content.unwrap(),
        }
    }
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

    let queries = to_queries(
        QueryParser::parse(
            Rule::queries,
            "…item 1
  id = id
  content = content",
        )
        .unwrap(),
    );

    let result: Vec<Item> = execute(queries, tree);
    println!(">> {:?}", result);
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
