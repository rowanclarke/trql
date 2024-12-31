use std::rc::Rc;

use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;
use trql::tree;

#[derive(Parser)]
#[grammar = "tree.pest"]
pub struct TreeParser;

pub fn to_tree(pairs: Pairs<Rule>) -> Tree {
    let mut nodes = Vec::new();
    pairs.for_each(|pair| add_node(pair, &mut nodes));
    Tree {
        index: 0,
        end: nodes.len(),
        nodes: nodes.into(),
    }
}

pub fn add_node<'a>(pair: Pair<'a, Rule>, nodes: &mut Vec<NodeData<'a>>) {
    let mut pairs = pair.into_inner();
    let index = nodes.len();
    nodes.push(NodeData {
        name: pairs.next().unwrap().as_str(),
        value: pairs.next().unwrap().as_str(),
        size: 0,
    });
    pairs.for_each(|pair| add_node(pair, nodes));
    let size = nodes.len() - index - 1;
    nodes[index].size = size;
}

#[derive(Clone, Debug)]
pub struct Tree<'a> {
    nodes: Rc<[NodeData<'a>]>,
    index: usize,
    end: usize,
}

#[derive(Clone)]
pub struct FlatTree<'a> {
    nodes: Rc<[NodeData<'a>]>,
    index: usize,
    end: usize,
}

#[derive(Clone)]
pub struct Node<'a> {
    nodes: Rc<[NodeData<'a>]>,
    index: usize,
}

#[derive(Clone, Debug)]
pub struct NodeData<'a> {
    name: &'a str,
    value: &'a str,
    size: usize,
}

impl<'a> Iterator for Tree<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.end {
            let node = Node {
                nodes: self.nodes.clone(),
                index: self.index,
            };
            self.index += self.nodes[self.index].size + 1;
            Some(node)
        } else {
            None
        }
    }
}

impl<'a> Iterator for FlatTree<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.end {
            let node = Node {
                nodes: self.nodes.clone(),
                index: self.index,
            };
            self.index += 1;
            Some(node)
        } else {
            None
        }
    }
}

impl<'a> tree::Tree for Tree<'a> {
    type Node = Node<'a>;
}

impl<'a> tree::FlatTree for FlatTree<'a> {
    type Node = Node<'a>;
}

impl<'a> tree::Node for Node<'a> {
    type Tree = Tree<'a>;

    type FlatTree = FlatTree<'a>;

    fn name(&self) -> &str {
        self.nodes[self.index].name
    }

    fn value(&self) -> Option<&str> {
        Some(self.nodes[self.index].value)
    }

    fn tree(self) -> Self::Tree {
        let NodeData { size, .. } = self.nodes[self.index];
        Tree {
            nodes: self.nodes,
            end: self.index + size + 1,
            index: self.index + 1,
        }
    }

    fn flat_tree(self) -> Self::FlatTree {
        let NodeData { size, .. } = self.nodes[self.index];
        FlatTree {
            nodes: self.nodes,
            end: self.index + size + 1,
            index: self.index,
        }
    }
}
