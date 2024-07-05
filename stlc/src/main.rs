#![feature(trait_alias)]
use chumsky::Parser;
mod ast;
mod parser;
mod typechecker;

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    match parser::parser().parse(src.clone()) {
        Ok(ast) => println!("{:#?}", ast),
        Err(parse_error) => parse_error
            .into_iter()
            .for_each(|e| println!("{:?}", e.reason(),)),
    }
}
