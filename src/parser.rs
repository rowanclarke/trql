use pest::iterators::{Pair, Pairs};

#[derive(Parser)]
#[grammar = "grammar.pest"] // relative to src
pub struct QueryParser;

#[derive(Debug)]
pub struct Query {
    name: Option<String>,
    select: Select,
    subqueries: Vec<Query>,
}

pub type Select = Vec<Series>;

pub type Series = Vec<Operation>;

#[derive(Debug)]
pub enum Operation {
    Parallel(Select),
    Condition(Select),
    Range { from: isize, to: isize, step: isize },
    Index(isize),
    Descendants,
    Children,
    Token(String),
}

pub fn to_queries(pairs: Pairs<Rule>) -> Vec<Query> {
    pairs.map(to_query).collect()
}

pub fn to_query(pair: Pair<Rule>) -> Query {
    let mut pairs = pair.into_inner();
    match pairs.next() {
        Some(n) if n.as_rule() == Rule::name => Query {
            name: Some(n.as_str().to_owned()),
            select: to_select(pairs.next().unwrap()),
            subqueries: to_queries(pairs),
        },
        Some(s) => Query {
            name: None,
            select: to_select(s),
            subqueries: to_queries(pairs),
        },
        _ => unreachable!(),
    }
}

pub fn to_select(pair: Pair<Rule>) -> Select {
    pair.into_inner().map(to_series).collect()
}

pub fn to_series(pair: Pair<Rule>) -> Series {
    pair.into_inner().map(to_operation).collect()
}

pub fn to_operation(pair: Pair<Rule>) -> Operation {
    match pair.as_rule() {
        Rule::parallel => Operation::Parallel(to_select(pair.into_inner().next().unwrap())),
        Rule::condition => Operation::Condition(to_select(pair.into_inner().next().unwrap())),
        Rule::range => {
            let (from, to, step) =
                pair.into_inner()
                    .fold((0, -1, 1), |(mut from, mut to, mut step), p| {
                        let i: isize = p.as_str().parse().unwrap();
                        match p.as_rule() {
                            Rule::from => from = i,
                            Rule::to => to = i,
                            Rule::step => step = i,
                            _ => unreachable!(),
                        };
                        (from, to, step)
                    });
            Operation::Range { from, to, step }
        }
        Rule::index => Operation::Index(pair.as_str().parse().unwrap()),
        Rule::descendants => Operation::Descendants,
        Rule::children => Operation::Children,
        Rule::token => Operation::Token(pair.as_str().to_owned()),
        _ => unreachable!(),
    }
}
