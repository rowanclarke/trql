use parser::{Operation, Select, Series};
use tree::{Chain, Children, Condition, Descendants, DynNodes, Node, Nodes, Tree};

#[macro_use]
extern crate pest_derive;

mod parser;
mod tree;

#[cfg(test)]
mod tests;

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
            Self::Descendants => Box::new(Descendants::<T, I>::new(iter)),
            Self::Children => Box::new(Children::<T, I>::new(iter)),
            Self::Token(token) => Box::new(iter.filter(move |node| node.name() == token)),
            _ => todo!(),
        }
    }
}
