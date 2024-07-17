#![feature(trait_alias)]
mod ast;
mod error;
mod inference;
mod parser;
use im::HashMap;

use crate::inference::Inference;
use chumsky::Parser;
use yansi::Paint;

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    match parser::parser().parse(src.clone()) {
        Ok(ast) => {
            let mut inference = Inference::new();
            match inference.infer(ast, HashMap::new()) {
                Ok(t) => {
                    inference.solve_constraints().unwrap_or_else(|e| {
                        e.report(&std::env::args().nth(1).unwrap());
                        inference.debug();
                        std::process::exit(1);
                    });

                    println!("{}", inference.substitute(t).to_string().green().bold());
                }
                Err(e) => e.report(&std::env::args().nth(1).unwrap()),
            }
        }
        Err(parse_error) => parse_error
            .into_iter()
            .for_each(|e| println!("{:?}", e.red().bold())),
    }
}
