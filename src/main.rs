mod ast;
mod lexer;
mod parser;
mod tests;
mod token;

use ast::*;
use lexer::Lexer;
use parser::Parser;
use std::io::BufRead;

#[macro_use]
extern crate lazy_static;

/// Parse a SQL query string into an AST
pub(crate) fn parse_sql(sql: &str) -> Result<Statement, String> {
    let lexer = Lexer::new(sql);
    let mut parser = Parser::new(lexer);
    parser.parse()
}

fn main() {
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let s = parse_sql(&line).unwrap();
        println!("{:?}", &s)
    }
}
