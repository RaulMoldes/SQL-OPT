mod ast;
mod lexer;
mod parser;
#[cfg(test)]
mod tests;
mod token;
//mod optimizer;
mod simplify;
mod visitor;
use ast::*;
use lexer::Lexer;
use parser::Parser;
use visitor::Visitor;
use simplify::Simplify;
use colored::*;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

use crate::simplify::Simplifyer;

/// Parse a SQL query string into an AST
pub(crate) fn parse_sql(sql: &str) -> Result<Statement, String> {
    let lexer = Lexer::new(sql);
    let mut parser = Parser::new(lexer);
    parser.visit()

}


/// Simplify a SQL query string into an optimized AST
pub(crate) fn simplify_sql(sql: &str) -> Result<Statement, String> {
    let lexer = Lexer::new(sql);
    let parser = Parser::new(lexer);
    let mut simplifier = Simplifyer::new(parser);
    simplifier.visit()

}

fn main() {
    let mut rl = DefaultEditor::new().unwrap();
    println!("{}", "ANSI-Compatible SQL Parser".bold().blue());
    println!("Type your SQL statements (end with ';'), or type .help or .exit.\n");

    loop {
        let readline = rl.readline("sql> ");
        match readline {
            Ok(mut line) => {
                line = line.trim().to_string();

                if line == ".exit" || line == ".quit" {
                    println!("{}", "Goodbye!".green());
                    break;
                }

                if line == ".help" {
                    println!(
                        "{}",
                        "Commands:\n  .help  - Show this message\n  .exit  - Quit the shell\n"
                            .bright_black()
                    );
                    continue;
                }

                while !line.trim_end().ends_with(';') {
                    let more = rl.readline("...> ");
                    match more {
                        Ok(next_line) => line.push_str(&format!(" {}", next_line)),
                        Err(_) => break,
                    }
                }

                if line.is_empty() {
                    continue;
                }

                rl.add_history_entry(line.clone()).ok();

                match simplify_sql(&line) {
                    Ok(ast) => {

                        println!("{}\n{:#?}\n", "Successfully parsed:".green().bold(), ast);
                    }
                    Err(err) => {
                        eprintln!("{} {}", "Parse error:".red().bold(), err);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C pressed. Type .exit to quit.");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("\n{}", "Goodbye!".green());
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }
}
