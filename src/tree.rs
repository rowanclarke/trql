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

#[derive(Clone)]
pub struct Children<T: Tree, I: Iterator<Item = T::Node>> {
    iter: I,
    child: Option<T>,
}

impl<T: Tree, I: Iterator<Item = T::Node>> Children<T, I> {
    pub fn new(mut iter: I) -> Self {
        Self {
            child: iter.next().map(Node::tree),
            iter,
        }
    }
}

impl<T: Tree, I: Iterator<Item = T::Node>> Iterator for Children<T, I> {
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

#[cfg(test)]
mod tests {}
