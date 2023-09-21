use crate::parser::{ParseError, Parser};
use std::env;

mod operation;
mod parser;

/// Defines the errors this application can throw
#[derive(Debug)]
enum ApplicationError {
    /// Error in the parse process
    Parser(ParseError),
    /// Illegal arguments passed to the program
    IllegalArgs,
}

fn main() -> Result<(), ApplicationError> {
    env_logger::init();

    // Show help if no argument is passed
    let mut args = env::args();
    let bin_path = args.next().unwrap_or(env!("CARGO_PKG_NAME").to_string());
    if args.len() < 1 {
        println!(
            "{} {} - Usage: {} <expression>",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            bin_path
        );
    }
    // If some expression is present, instantiate the parse and attempt to parse it
    if let Some(expression) = args.next() {
        let parser = Parser::new(expression);
        let result = parser.parse().map_err(ApplicationError::Parser)?;
        println!("{}", result);
        Ok(())
    } else {
        Err(ApplicationError::IllegalArgs)
    }
}
