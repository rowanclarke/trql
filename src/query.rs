use std::{marker::PhantomData, rc::Rc};

use crate::{
    command::Command,
    tree::{DynNodes, Node, Tree},
};

pub fn execute<T: Tree + 'static, I: FromNodes<T>>(queries: Vec<Query>, tree: T) -> I {
    I::from_nodes(queries, Box::new(tree) as Box<dyn DynNodes<T>>)
}

pub trait FromNodes<T: Tree>: Sized {
    fn from_nodes(queries: Vec<Query>, nodes: Box<dyn DynNodes<T>>) -> Self;
}

impl<T: Tree + 'static, I: FromNodes<T>> FromNodes<T> for Vec<I> {
    fn from_nodes(queries: Vec<Query>, nodes: Box<dyn DynNodes<T>>) -> Self {
        queries
            .into_iter()
            .flat_map(|query| {
                let (select, subqueries) = query.into_unnamed().unwrap();
                select
                    .execute::<T, _>(nodes.clone())
                    .map(move |node| I::from_nodes(subqueries.clone(), Box::new(node.tree())))
            })
            .collect()
    }
}

impl<T: Tree> FromNodes<T> for String {
    fn from_nodes(_: Vec<Query>, nodes: Box<dyn DynNodes<T>>) -> Self {
        nodes.fold(String::new(), |s, node| s + node.value().unwrap())
    }
}

#[derive(Debug, Clone)]
pub struct Query {
    name: Option<String>,
    select: Select,
    subqueries: Vec<Query>,
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

impl Query {
    pub fn new(name: Option<String>, select: Select, subqueries: Vec<Query>) -> Self {
        Self {
            name,
            select,
            subqueries,
        }
    }

    pub fn into_named(self) -> Option<(String, Select, Vec<Query>)> {
        match self.name {
            Some(s) => Some((s, self.select, self.subqueries)),
            None => None,
        }
    }

    pub fn into_unnamed(self) -> Option<(Select, Vec<Query>)> {
        match self.name {
            Some(_) => None,
            None => Some((self.select, self.subqueries)),
        }
    }
}
