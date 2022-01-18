extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar = "plonk_ir.pest"]
struct PlonkIRParser;

pub(crate) fn print_pair(level: usize, pair: Pair<Rule>) {
    println!("{}{:?}: {}", "  ".repeat(level), pair.as_rule(), pair.as_str());

    for pair in pair.into_inner() {
        print_pair(level + 1, pair)
    }

}

fn main() {
    let pairs = PlonkIRParser::parse(Rule::circuit, "pub a1 b2\na1 b2^2 * c3 + d4 \n def gate a -> b {\n a (b 1) c \n e d 2 \n}").unwrap_or_else(|e| panic!("{}", e));

    for pair in pairs {
        print_pair(0, pair);
    }
}