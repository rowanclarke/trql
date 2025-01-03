use std::collections::BTreeMap;

use pest::iterators::{Pair, Pairs};

use crate::query::{Operation, Queries, Query, Select, Series};

#[derive(Parser)]
#[grammar = "grammar.pest"] // relative to src
pub struct QueryParser;

pub fn to_queries(pairs: Pairs<Rule>) -> Queries {
    let mut map: Queries = BTreeMap::new();
    pairs
        .map(to_query)
        .for_each(|(name, query)| map.entry(name).or_insert(vec![]).push(query));
    map
}

fn to_query(pair: Pair<Rule>) -> (Option<String>, Query) {
    let mut pairs = pair.into_inner();
    match pairs.next() {
        Some(n) if n.as_rule() == Rule::name => (
            Some(n.as_str().to_owned()),
            Query::new(to_select(pairs.next().unwrap()), to_queries(pairs)),
        ),
        Some(s) => (None, Query::new(to_select(s), to_queries(pairs))),
        _ => unreachable!(),
    }
}

fn to_select(pair: Pair<Rule>) -> Select {
    pair.into_inner().map(to_series).collect()
}

fn to_series(pair: Pair<Rule>) -> Series {
    pair.into_inner().map(to_operation).collect()
}

fn to_operation(pair: Pair<Rule>) -> Operation {
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
        Rule::index => {
            let index = pair.as_str().parse().unwrap();
            Operation::Range {
                from: index,
                to: index,
                step: 1,
            }
        }
        Rule::descendants => Operation::Descendants,
        Rule::children => Operation::Children,
        Rule::token => Operation::Token(pair.as_str().to_owned()),
        _ => unreachable!(),
    }
}
