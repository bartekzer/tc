mod parser;

use crate::parser::parser::NumParser;

fn main() {
    let parser = NumParser::new();
    match parser.parse("246810") {
        Ok(n) => println!("Parsed number: {}", n),
        Err(e) => println!("Error: {}", e),
    }
}