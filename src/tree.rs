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

#[cfg(test)]
mod tests {}
