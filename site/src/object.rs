use std::collections::BTreeMap;

use trql::{
    query::{Command, Queries, Query, QueryResult},
    tree::{DynNodes, Node, Tree},
};

pub struct ObjectString(pub String);

impl<'a, T: Tree + 'a> QueryResult<'a, T> for ObjectString {
    fn from_nodes(queries: Queries, nodes: Box<dyn DynNodes<T> + 'a>) -> Self {
        let objects: BTreeMap<Option<String>, String> = queries
            .into_iter()
            .map(|(name, queries)| {
                let mut result = if let Some(name) = &name {
                    format!("{}: [", name)
                } else {
                    format!("[")
                };
                queries
                    .into_iter()
                    .for_each(|Query { select, subqueries }| {
                        select
                            .clone()
                            .execute::<T, _>(nodes.clone())
                            .enumerate()
                            .for_each(|(i, node)| {
                                if i > 0 {
                                    result += ",";
                                }
                                result +=
                                    &<Self as QueryResult<T>>::from_node(subqueries.clone(), node)
                                        .0;
                            });
                    });
                result += "]";
                (name, result)
            })
            .collect();
        if let Some(result) = objects.get(&None) {
            Self(result.clone())
        } else {
            Self(format!(
                "{{{}}}",
                objects
                    .into_values()
                    .enumerate()
                    .fold(String::new(), |s, (i, result)| s
                        + if i > 0 { "," } else { "" }
                        + &result)
            ))
        }
    }

    fn from_leaf(leaf: T::Node) -> Self {
        Self(leaf.value().unwrap().to_owned())
    }
}
