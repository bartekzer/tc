#![feature(trait_alias)]
use chumsky::Parser;
use yansi::Paint;
mod ast;
mod error;
mod parser;
mod typechecker;

use typechecker::*;

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    match parser::parser().parse(src.clone()) {
        Ok(ast) => {
            // println!("{:#?}", ast);

            let mut tc = TypeChecker::new();
            tc.infer(ast)
                .map(|typ| println!("{}", typ.to_string().green().bold()))
                .unwrap_or_else(|e| e.report(&std::env::args().nth(1).unwrap()));
        }
        Err(parse_error) => parse_error
            .into_iter()
            .for_each(|e| println!("{:?}", e.reason().red().bold())),
    }
}
