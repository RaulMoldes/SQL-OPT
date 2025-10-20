mod ast;
mod lexer;
mod parser;
#[cfg(test)]
mod tests;
mod token;
mod visitor;

use ast::*;
use lexer::Lexer;
use parser::Parser;
use visitor::SQLParser;


use colored::*;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

/// Parse a SQL query string into an AST
pub(crate) fn parse_sql(sql: &str) -> Result<Statement, String> {
    let lexer = Lexer::new(sql);
    let mut parser = Parser::new(lexer);
    parser.parse()
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
                        "Commands:\n  .help  - Show this message\n  .exit  - Quit the shell\n".bright_black()
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

                match parse_sql(&line) {
                    Ok(ast) => {
                        println!(
                            "{}\n{:#?}\n",
                            "Successfully parsed:".green().bold(),
                            ast
                        );
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
