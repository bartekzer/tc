#![feature(trait_alias)]
use chumsky::Parser;
mod ast;
mod parser;

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    println!("\n\n\n\n{}\n\n\n\n", src);

    match parser::parser().parse(src.clone()) {
        Ok(ast) => println!("{:#?}", ast),
        Err(parse_error) => parse_error.into_iter().for_each(|e| println!("{e}")),
    }
}
