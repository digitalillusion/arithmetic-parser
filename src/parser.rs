use std::iter::Peekable;
use crate::parser::ParseError::{EmptyExpression, IllegalState, UnbalancedParenthesis};
use log::{debug, trace};
use std::str::Chars;

use crate::operation::{codes::*, Operation, OperationError};

/// Errors that the parsing process can cause
#[derive(Debug, PartialEq)]
pub enum ParseError {
    /// The expression to parse is empty
    EmptyExpression,
    /// There is an error converting an operand from string to unsigned integer (operand, error message)
    ParseDigitError(String, String),
    /// The instantiation or application of an operation failed (`OperationError` for further information)
    InvalidOperation(OperationError),
    /// The expression is not arithmetically correct (invalid character)
    MalformedExpression(String),
    /// The number of parenthesis in the expression does not equal (open/close parenthesis operation code to indicate)
    UnbalancedParenthesis(String),
    /// The parser encountered an unexpected symbol (unexpected character, parser state, current operation)
    UnexpectedSymbol(String, ParserState, Option<Operation>),
    /// The parser ended in an illegal state
    IllegalState(String),
}

/// The legal states the parser can go through
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParserState {
    /// The first operand is expected
    FirstOperand,
    /// An operation code is expected
    Operation,
    /// The second operand is expected
    SecondOperand,
    /// A closing parenthesis is expected
    CloseParenthesis,
}

/// The parser structure
pub struct Parser {
    /// The expression to parse
    expression: String,
}

/// The parser implementation
impl Parser {
    /// Instantiate a new parser
    /// # Arguments
    ///  - expression: The expression to aprse
    /// # Return
    /// A `Parser`
    pub fn new(expression: String) -> Self {
        Self {
            expression,
        }
    }

    /// Parse process
    /// # Return
    /// A `Result` having the expression result if valid, `ParseError` otherwise
    pub fn parse(&self) -> Result<usize, ParseError> {
        let mut data: Peekable<Chars> = self.expression.chars().peekable();
        let open_brackets = data.clone().filter(|c| *c == OPCODE_OPEN).count();
        let closed_brackets = data.clone().filter(|c| *c == OPCODE_CLOSE).count();
        match (open_brackets, closed_brackets) {
            (open_brackets, closed_brackets) if open_brackets > closed_brackets => Err(UnbalancedParenthesis(OPCODE_OPEN.to_string())),
            (open_brackets, closed_brackets) if closed_brackets > open_brackets => Err(UnbalancedParenthesis(OPCODE_CLOSE.to_string())),
            _ => {
                let mut result = None;
                while data.clone().count() > 0 {
                    let res = self.parse_internal(&mut data, result)?;
                    result = Some(res);
                }
                result.ok_or(EmptyExpression)
            }
        }

    }

    /// Internal, recursive parse function
    fn parse_internal(
        &self,
        data: &mut Peekable<Chars>,
        mut result: Option<usize>,
    ) -> Result<usize, ParseError> {
        trace!("parse_internal() recursion");

        let mut state = ParserState::FirstOperand;
        let mut operation: Option<Operation> = None;
        let mut acc = String::new();
        while let Some(char) = data.next() {
            let is_digit = char.is_ascii_digit();
            let new_state = self.compute_state(state, char.to_owned(), &mut acc)?;
            if state != new_state {
                trace!("{:?} -> {:?}", state, new_state);
                state = new_state;
            }

            match char {
                char if state == ParserState::FirstOperand && is_digit.to_owned() => {
                    acc.push_str(&char.to_string());
                    trace!("a = {:?}", acc);
                    result = Some(acc.parse::<usize>().map_err(|err| {
                        ParseError::ParseDigitError(acc.clone(), err.to_string())
                    })?);
                }
                char if state == ParserState::SecondOperand && is_digit.to_owned() => {
                    acc.push_str(&char.to_string());
                    trace!("b = {:?}", acc);
                    result = Some(
                        operation
                            .ok_or(IllegalState(
                                "No operation when evaluating SecondOperand".to_string(),
                            ))?
                            .apply(acc.to_string())
                            .map_err(ParseError::InvalidOperation)?,
                    );
                }
                code @ (OPCODE_ADD | OPCODE_SUB | OPCODE_MUL | OPCODE_DIV)
                    if state == ParserState::Operation =>
                {
                    operation = if acc.is_empty() {
                        let first_operand = result.ok_or(ParseError::IllegalState(
                            "No previous result and accumulator empty instantiating operation"
                                .to_string(),
                        ))?;
                        Some(
                            Operation::from_result(code, first_operand)
                                .map_err(ParseError::InvalidOperation)?,
                        )
                    } else {
                        Some(
                            Operation::from(code, acc.to_string())
                                .map_err(ParseError::InvalidOperation)?,
                        )
                    };
                    trace!("op = {:?}", operation);
                    acc.clear();
                }
                OPCODE_OPEN => {
                    trace!(
                        "Open Parenthesis: state = {:?}, operation = {:?}",
                        state,
                        operation
                    );
                    let res = match operation {
                        None => self.parse_internal(data, result),
                        Some(operation) => operation
                            .apply_result(self.parse_internal(data, result)?)
                            .map_err(ParseError::InvalidOperation),
                    };
                    match data.peek().cloned() {
                        Some(OPCODE_ADD) | Some(OPCODE_SUB) | Some(OPCODE_MUL) | Some(OPCODE_DIV) => {
                            result = res.ok();
                            state = ParserState::FirstOperand;
                        },
                        _ => return res,
                    }
                }
                OPCODE_CLOSE if state == ParserState::CloseParenthesis => {
                    trace!(
                        "Close Parenthesis, operation={:?}, result = {:?}",
                        operation,
                        result,
                    );
                    return result.ok_or(IllegalState(
                        "Result not available when closing parenthesis".to_string(),
                    ));
                }
                symbol => {
                    return Err(ParseError::UnexpectedSymbol(
                        symbol.to_string(),
                        state,
                        operation,
                    ))
                }
            }
        }

        debug!("result = {:?}", &result);
        result.ok_or(EmptyExpression)
    }

    /// Compute the new state of the parser
    fn compute_state(
        &self,
        state: ParserState,
        char: char,
        acc: &mut String,
    ) -> Result<ParserState, ParseError> {
        let is_digit = char.is_ascii_digit();
        match state {
            ParserState::FirstOperand if !is_digit.to_owned() => match char {
                OPCODE_ADD | OPCODE_SUB | OPCODE_MUL | OPCODE_DIV => {
                    acc.clear();
                    Ok(ParserState::Operation)
                }
                OPCODE_OPEN => Ok(ParserState::FirstOperand),
                OPCODE_CLOSE => {
                    acc.clear();
                    Ok(ParserState::CloseParenthesis)
                }
                _ => Err(ParseError::MalformedExpression(char.to_string())),
            },
            ParserState::Operation if is_digit.to_owned() => Ok(ParserState::SecondOperand),
            ParserState::Operation if !is_digit.to_owned() => match char {
                OPCODE_ADD | OPCODE_SUB | OPCODE_MUL | OPCODE_DIV if !acc.is_empty() => {
                    acc.clear();
                    Ok(state)
                }
                OPCODE_OPEN => {
                    acc.clear();
                    Ok(state)
                }
                _ => Err(ParseError::MalformedExpression(char.to_string())),
            },
            ParserState::SecondOperand if !is_digit.to_owned() => match char {
                OPCODE_ADD | OPCODE_SUB | OPCODE_MUL | OPCODE_DIV => {
                    acc.clear();
                    Ok(ParserState::Operation)
                }
                OPCODE_OPEN => Ok(ParserState::SecondOperand),
                OPCODE_CLOSE => {
                    acc.clear();
                    Ok(ParserState::CloseParenthesis)
                }
                _ => Err(ParseError::MalformedExpression(char.to_string())),
            },
            ParserState::CloseParenthesis if !is_digit.to_owned() => match char {
                OPCODE_ADD | OPCODE_SUB | OPCODE_MUL | OPCODE_DIV => {
                    acc.clear();
                    Ok(ParserState::Operation)
                }
                OPCODE_CLOSE => Ok(ParserState::CloseParenthesis),
                _ => Err(ParseError::UnbalancedParenthesis(char.to_string())),
            },
            ParserState::FirstOperand | ParserState::SecondOperand if is_digit.to_owned() => {
                Ok(state)
            }
            _ => Err(ParseError::MalformedExpression(char.to_string())),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::operation::OperationError::OverflowError;
    use crate::parser::ParseError::{
        EmptyExpression, InvalidOperation, MalformedExpression, ParseDigitError,
        UnbalancedParenthesis,
    };
    use crate::parser::Parser;

    #[test]
    fn test_examples() {
        let expression = "3a2c4".to_string();
        let parser = Parser::new(expression);
        let result = parser.parse().unwrap();
        assert_eq!(20, result);

        let expression = "32a2d2".to_string();
        let parser = Parser::new(expression);
        let result = parser.parse().unwrap();
        assert_eq!(17, result);

        let expression = "500a10b66c32".to_string();
        let parser = Parser::new(expression);
        let result = parser.parse().unwrap();
        assert_eq!(14208, result);

        let expression = "3ae4c66fb32".to_string();
        let parser = Parser::new(expression);
        let result = parser.parse().unwrap();
        assert_eq!(235, result);

        let expression = "3c4d2aee2a4c41fc4f".to_string();
        let parser = Parser::new(expression);
        let result = parser.parse().unwrap();
        assert_eq!(990, result);
    }

    #[test]
    fn test_redundant_parenthesis() {
        let expression = "e2f".to_string();
        let parser = Parser::new(expression);
        let result = parser.parse().unwrap();
        assert_eq!(2, result);

        let expression = "e2fae3f".to_string();
        let parser = Parser::new(expression);
        let result = parser.parse().unwrap();
        assert_eq!(5, result);
    }

    #[test]
    fn test_malformed() {
        let expression = "3aa2c4".to_string();
        let parser = Parser::new(expression);
        let result = parser.parse();
        assert_eq!(Err(MalformedExpression("a".to_string())), result);
    }

    #[test]
    fn test_unbalanced_parenthesis() {
        let expression = "3aee2fc4".to_string();
        let parser = Parser::new(expression);
        let result = parser.parse();
        assert_eq!(Err(UnbalancedParenthesis("e".to_string())), result);

        let expression = "3aee2fffc4".to_string();
        let parser = Parser::new(expression);
        let result = parser.parse();
        assert_eq!(Err(UnbalancedParenthesis("f".to_string())), result);
    }

    #[test]
    fn test_nested_parenthesis() {
        let expression = "233b3ae4c66fb99ae33ce3a5ff".to_string();
        let parser = Parser::new(expression);
        let result = parser.parse().unwrap();
        assert_eq!(659, result);

        let expression = "eeee5fae3fffcee2fff".to_string();
        let parser = Parser::new(expression);
        let result = parser.parse().unwrap();
        assert_eq!(16, result);
    }

    #[test]
    fn test_overflow() {
        let expression = "99999999999999999999999999c9".to_string();
        let parser = Parser::new(expression);
        let result = parser.parse();
        assert_eq!(
            Err(ParseDigitError(
                "99999999999999999999".to_string(),
                "number too large to fit in target type".to_string()
            )),
            result
        );

        let expression = "9c99999999999999999999999999".to_string();
        let parser = Parser::new(expression);
        let result = parser.parse();
        assert_eq!(Err(InvalidOperation(OverflowError)), result);
    }

    #[test]
    fn test_empty() {
        let expression = "".to_string();
        let parser = Parser::new(expression);
        let result = parser.parse();
        assert_eq!(Err(EmptyExpression), result);
    }
}
