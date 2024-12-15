use crate::tree::*;
use trql_derive;

use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct TestNodeData {
    name: String,
    value: Option<String>,
    len: usize,
}

#[derive(Clone, Debug)]
pub struct TestNode {
    data: Rc<Vec<TestNodeData>>,
    index: usize,
}

#[derive(Clone)]
pub struct TestTree {
    data: Rc<Vec<TestNodeData>>,
    end: usize,
    index: usize,
}

#[derive(Clone)]
pub struct TestFlatTree {
    data: Rc<Vec<TestNodeData>>,
    end: usize,
    index: usize,
}

impl Node for TestNode {
    type Tree = TestTree;

    type FlatTree = TestFlatTree;

    fn name(&self) -> &str {
        self.data[self.index].name.as_ref()
    }

    fn value(&self) -> Option<&str> {
        self.data[self.index].value.as_deref()
    }

    fn tree(self) -> Self::Tree {
        let TestNodeData { len, .. } = self.data[self.index];
        TestTree {
            data: self.data,
            end: self.index + len + 1,
            index: self.index + 1,
        }
    }

    fn flat_tree(self) -> Self::FlatTree {
        let TestNodeData { len, .. } = self.data[self.index];
        TestFlatTree {
            data: self.data,
            end: self.index + len + 1,
            index: self.index,
        }
    }
}

impl Tree for TestTree {
    type Node = TestNode;
}

impl FlatTree for TestFlatTree {
    type Node = TestNode;
}

impl Iterator for TestTree {
    type Item = TestNode;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.end {
            let node = TestNode {
                data: self.data.clone(),
                index: self.index,
            };
            self.index += self.data[self.index].len + 1;
            Some(node)
        } else {
            None
        }
    }
}

impl Iterator for TestFlatTree {
    type Item = TestNode;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.end {
            let node = TestNode {
                data: self.data.clone(),
                index: self.index,
            };
            self.index += 1;
            Some(node)
        } else {
            None
        }
    }
}

impl TestNodeData {
    pub fn new(name: &str, value: Option<&str>, len: usize) -> Self {
        Self {
            name: name.to_string(),
            value: value.map(|s| s.to_string()),
            len,
        }
    }
}

impl TestTree {
    pub fn new(data: Rc<Vec<TestNodeData>>) -> Self {
        Self {
            index: 0,
            end: data.len(),
            data,
        }
    }
}
