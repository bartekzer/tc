#![feature(trait_alias)]
mod ast;
mod parser;
use chumsky::Parser;
use yansi::Paint;

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    match parser::parser().parse(src.clone()) {
        Ok(ast) => {
            println!("{:#?}", ast);
        }
        Err(parse_error) => parse_error
            .into_iter()
            .for_each(|e| println!("{:?}", e.red().bold())),
    }
}
