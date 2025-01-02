use serde::{Serialize, Serializer};
use std::{
    array,
    collections::{BTreeMap, HashMap},
};

use trql::{
    query::{Command, Queries, Query, QueryResult},
    tree::{DynNodes, Node, Tree},
};

#[derive(Clone)]
pub enum Object {
    Array(Vec<Object>),
    Map(BTreeMap<String, Vec<Object>>),
    Value(String),
}

impl Serialize for Object {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Object::Array(vec) => vec.serialize(serializer),
            Object::Map(map) => map.serialize(serializer),
            Object::Value(value) => value.serialize(serializer),
        }
    }
}

impl<'a, T: Tree + 'a> QueryResult<'a, T> for Object {
    fn from_nodes(queries: Queries, nodes: Box<dyn DynNodes<T> + 'a>) -> Self {
        let mut objects: BTreeMap<String, Vec<Object>> = BTreeMap::new();

        for (name, queries) in queries {
            let result: Vec<Object> = queries
                .into_iter()
                .flat_map(|Query { select, subqueries }| {
                    select
                        .clone()
                        .execute::<T, _>(nodes.clone())
                        .map(move |node| {
                            <Self as QueryResult<T>>::from_node(subqueries.clone(), node)
                        })
                })
                .collect();
            match name {
                Some(name) => {
                    objects.insert(name, result);
                }
                None => {
                    return Object::Array(result);
                }
            }
        }

        Object::Map(objects)
    }

    fn from_leaf(leaf: <T as Tree>::Node) -> Self {
        Object::Value(<String as QueryResult<T>>::from_leaf(leaf))
    }
}
