#[macro_use]
extern crate pest_derive;

mod parser;

#[cfg(test)]
mod tests {
    use pest::Parser;

    use crate::parser::{QueryParser, Rule};

    use super::parser::to_queries;

    #[test]
    fn query() {
        let queries = to_queries(
            QueryParser::parse(
                Rule::queries,
                "…node
  id = id
  content = content",
            )
            .unwrap(),
        );
        println!("{:?}", queries);
    }
}
