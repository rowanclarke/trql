use crate::{
    query::{Operation, Select, Series},
    tree::{Chain, Children, Condition, Descendants, DynNodes, Node, Nodes, Tree},
};

pub trait Command {
    fn execute<T: Tree + 'static, I: Nodes<T> + 'static>(self, iter: I) -> Box<dyn DynNodes<T>>;
}

impl Command for Select {
    fn execute<T: Tree + 'static, I: Nodes<T> + 'static>(self, iter: I) -> Box<dyn DynNodes<T>> {
        Box::new(Chain::<T>::new(
            self.into_iter()
                .map(|x| x.execute::<T, I>(iter.clone()))
                .collect(),
        ))
    }
}

impl Command for Series {
    fn execute<T: Tree + 'static, I: Nodes<T> + 'static>(self, iter: I) -> Box<dyn DynNodes<T>> {
        let mut iter = Box::new(iter) as Box<dyn DynNodes<T>>;
        for command in self {
            iter = command.execute::<T, Box<dyn DynNodes<T>>>(iter);
        }
        iter
    }
}

impl Command for Operation {
    fn execute<T: Tree + 'static, I: Nodes<T> + 'static>(self, iter: I) -> Box<dyn DynNodes<T>> {
        match self {
            Self::Parallel(select) => select.execute(iter),
            Self::Condition(select) => Box::new(Condition::new(iter, move |tree: T| {
                select.clone().execute::<T, T>(tree)
            })),
            Self::Range { from, to, step } => Box::new(
                iter.zip(1..)
                    .filter(move |&(_, i)| (from <= i && i <= to && (i - from) % step == 0))
                    .map(|(node, _)| node),
            ),
            Self::Descendants => Box::new(Descendants::<T, I>::new(iter)),
            Self::Children => Box::new(Children::<T, I>::new(iter)),
            Self::Token(token) => Box::new(iter.filter(move |node| node.name() == token)),
        }
    }
}
