use dyn_clone::DynClone;
use std::marker::PhantomData;

use crate::chain;

pub trait Node: Clone {
    type Tree: Tree<Node = Self>;
    type FlatTree: FlatTree<Node = Self>;

    fn name(&self) -> &str;
    fn value(&self) -> Option<&str>;
    fn tree(self) -> Self::Tree;
    fn flat_tree(self) -> Self::FlatTree;
}

pub trait Tree: Clone + Iterator<Item = Self::Node> {
    type Node: Node<Tree = Self>;
}

pub trait FlatTree: Clone + Iterator<Item = Self::Node> {
    type Node: Node<FlatTree = Self>;
}

pub trait Nodes<T: Tree>: Clone + DynNodes<T> {}

pub trait DynNodes<T: Tree>: DynClone + Iterator<Item = T::Node> {}

impl<T: Tree, I: Iterator<Item = T::Node> + Clone> Nodes<T> for I {}

impl<T: Tree, I: Iterator<Item = T::Node> + Clone> DynNodes<T> for I {}

impl<T: Tree> Clone for Box<dyn DynNodes<T>> {
    fn clone(&self) -> Self {
        dyn_clone::clone_box(&**self)
    }
}

pub type Chain<T> = chain::Chain<Box<dyn DynNodes<T>>>;

#[derive(Clone)]
pub struct Children<T: Tree, I: Nodes<T>> {
    iter: I,
    child: Option<T>,
}

impl<T: Tree, I: Nodes<T>> Children<T, I> {
    pub fn new(mut iter: I) -> Self {
        Self {
            child: iter.next().map(Node::tree),
            iter,
        }
    }
}

impl<T: Tree, I: Nodes<T>> Iterator for Children<T, I> {
    type Item = T::Node;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(tree) = &mut self.child {
            if let Some(node) = tree.next() {
                Some(node)
            } else {
                self.child = self.iter.next().map(|node| node.tree());
                self.next()
            }
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Descendants<T: Tree, I: Nodes<T>> {
    iter: I,
    flat_child: Option<<T::Node as Node>::FlatTree>,
}

impl<T: Tree, I: Nodes<T>> Descendants<T, I> {
    pub fn new(mut iter: I) -> Self {
        Self {
            flat_child: iter.next().map(Node::flat_tree),
            iter,
        }
    }
}

impl<T: Tree, I: Nodes<T>> Iterator for Descendants<T, I> {
    type Item = T::Node;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(tree) = &mut self.flat_child {
            if let Some(node) = tree.next() {
                Some(node)
            } else {
                self.flat_child = self.iter.next().map(|node| node.flat_tree());
                self.next()
            }
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Condition<
    T: Tree,
    I: Iterator<Item = T::Node>,
    J: Iterator<Item = T::Node>,
    F: FnMut(T) -> J,
> {
    iter: I,
    f: F,
    phantom: PhantomData<(T, J)>,
}

impl<T: Tree, I: Nodes<T>, J: Iterator<Item = T::Node>, F: FnMut(T) -> J> Condition<T, I, J, F> {
    pub fn new(iter: I, f: F) -> Self {
        Self {
            iter,
            f,
            phantom: PhantomData,
        }
    }
}

impl<T: Tree, I: Nodes<T>, J: Iterator<Item = T::Node>, F: FnMut(T) -> J> Iterator
    for Condition<T, I, J, F>
{
    type Item = T::Node;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .find(|node| (self.f)(node.clone().tree()).next().is_some())
    }
}
