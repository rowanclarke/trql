use parser::{Operation, Select};
use tree::{Children, Condition, Descendants, Tree};

#[macro_use]
extern crate pest_derive;

mod parser;
mod tree;

#[cfg(test)]
mod tests;

pub trait Command {
    fn execute<T: Tree + 'static, I: Iterator<Item = T::Node> + 'static>(
        self,
        iter: I,
    ) -> Box<dyn Iterator<Item = T::Node>>;
}

impl Command for Operation {
    fn execute<T: Tree + 'static, I: Iterator<Item = T::Node> + 'static>(
        self,
        iter: I,
    ) -> Box<dyn Iterator<Item = T::Node>> {
        match self {
            Self::Children => Box::new(Children::<T, I>::new(iter)),
            Self::Descendants => Box::new(Descendants::<T, I>::new(iter)),
            _ => todo!(),
        }
    }
}
