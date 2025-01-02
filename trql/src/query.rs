mod command;
mod parser;

use std::collections::BTreeMap;

use pest::Parser;

use crate::tree::{DynNodes, Node, Tree};
pub use command::Command;
use parser::{to_queries, QueryParser, Rule};

pub fn execute<'a, T: Tree + 'a, I: QueryResult<'a, T>>(queries: &str, tree: T) -> I {
    I::from_nodes(
        to_queries(QueryParser::parse(Rule::queries, queries).unwrap()),
        Box::new(tree),
    )
}

type Nodes<'a, T> = Box<dyn DynNodes<T> + 'a>;

pub trait QueryResult<'a, T: Tree + 'a>: Sized {
    fn from_nodes(queries: Queries, nodes: Nodes<'a, T>) -> Self;
    fn from_leaf(leaf: T::Node) -> Self;

    fn from_node(queries: Queries, node: <T as Tree>::Node) -> Self {
        if queries.is_empty() {
            Self::from_leaf(node)
        } else {
            Self::from_nodes(queries, Box::new(node.tree()))
        }
    }
}

impl<'a, T: Tree + 'a, I: QueryResult<'a, T>> QueryResult<'a, T> for Vec<I> {
    fn from_nodes(queries: Queries, nodes: Nodes<'a, T>) -> Self {
        queries
            .get(&None)
            .unwrap()
            .into_iter()
            .flat_map(|Query { select, subqueries }| {
                select
                    .clone()
                    .execute::<T, _>(nodes.clone())
                    .map(|node| I::from_node(subqueries.clone(), node))
            })
            .collect()
    }

    fn from_leaf(leaf: <T as Tree>::Node) -> Self {
        vec![I::from_leaf(leaf)]
    }
}

impl<'a, T: Tree + 'a> QueryResult<'a, T> for String {
    fn from_nodes(queries: Queries, nodes: Nodes<'a, T>) -> Self {
        queries.get(&None).unwrap().into_iter().fold(
            String::new(),
            |mut s, Query { select, subqueries }| {
                select
                    .clone()
                    .execute::<T, _>(nodes.clone())
                    .for_each(|node| {
                        s += &<Self as QueryResult<T>>::from_node(subqueries.clone(), node)
                    });
                s
            },
        )
    }

    fn from_leaf(node: T::Node) -> Self {
        node.value().unwrap().to_owned()
    }
}

pub type Queries = BTreeMap<Option<String>, Vec<Query>>;

#[derive(Debug, Clone)]
pub struct Query {
    pub select: Select,
    pub subqueries: Queries,
}

impl Query {
    pub fn new(select: Select, subqueries: Queries) -> Self {
        Self { select, subqueries }
    }
}

pub type Select = Vec<Series>;

pub type Series = Vec<Operation>;

#[derive(Debug, Clone)]
pub enum Operation {
    Parallel(Select),
    Condition(Select),
    Range { from: isize, to: isize, step: isize },
    Descendants,
    Children,
    Token(String),
}
