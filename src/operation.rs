use log::trace;

pub mod codes {
    /// Operation code for addition
    pub const OPCODE_ADD: char = 'a';
    /// Operation code for subtraction
    pub const OPCODE_SUB: char = 'b';
    /// Operation code for multiplication
    pub const OPCODE_MUL: char = 'c';
    /// Operation code for division
    pub const OPCODE_DIV: char = 'd';
    /// Operation code for open parenthesis
    pub const OPCODE_OPEN: char = 'e';
    /// Operation code for closed parenthesis
    pub const OPCODE_CLOSE: char = 'f';
}

use codes::*;

/// Errors that the Operation instantiation and application can cause
#[derive(Debug, PartialEq)]
pub enum OperationError {
    /// The first operand is invalid (character, error message)
    InvalidFirstOperand(String, String),
    /// The second operand is invalid (character, error message)
    InvalidSecondOperand(String, String),
    /// The operation code is invalid (invalid code)
    InvalidOperationCode(char),
    /// The operation application overflows
    OverflowError,
}

/// Enumeration of all possible arithmetical operations
#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    /// Addition (first operand)
    Add(usize),
    /// Subtraction (first operand)
    Sub(usize),
    /// Multiplication (first operand)
    Mul(usize),
    /// Division (first operand)
    Div(usize),
}

/// Implementation of an arithmetical operation
impl Operation {

    /// Creates the `Operation` from a code and the first operand
    /// # Arguments
    ///  - code: An char defined as `OPCODE` constant in the `codes` module
    ///  - first_operand: A string to be parsed as first operand of the operation
    /// # Return
    /// A `Result` having an `Operation` if valid, `OperationError` otherwise
    pub fn from(code: char, first_operand: String) -> Result<Self, OperationError> {
        let parsed = first_operand
            .parse::<usize>()
            .map_err(|err| OperationError::InvalidFirstOperand(first_operand, err.to_string()))?;
        trace!("parsed={}", parsed);
        Self::from_result(code, parsed)
    }

    /// Creates the `Operation` from a code and using a previous result as first operand
    /// # Arguments
    ///  - code: An char defined as `OPCODE` constant in the `codes` module
    ///  - first_operand: The previous result
    /// # Return
    /// A `Result` having an `Operation` if valid, `OperationError` otherwise
    pub fn from_result(code: char, first_operand: usize) -> Result<Self, OperationError> {
        match code {
            OPCODE_ADD => Ok(Operation::Add(first_operand)),
            OPCODE_SUB => Ok(Operation::Sub(first_operand)),
            OPCODE_MUL => Ok(Operation::Mul(first_operand)),
            OPCODE_DIV => Ok(Operation::Div(first_operand)),
            code => Err(OperationError::InvalidOperationCode(code)),
        }
    }

    /// Applies the `Operation` using a second operand
    /// # Arguments
    ///  - second_operand: A string to be parsed as second operand of the operation
    /// # Return
    /// A `Result` having a the arithmetic result of the operation if valid, `OperationError` otherwise
    pub fn apply(&self, second_operand: String) -> Result<usize, OperationError> {
        trace!("{:?} {}", self, second_operand);
        let parsed = second_operand
            .parse::<usize>()
            .map_err(|err| OperationError::InvalidSecondOperand(second_operand, err.to_string()))?;
        trace!("parsed={}", parsed);
        self.apply_result(parsed)
    }

    /// Applies the `Operation` using a previous result as second operand
    /// # Arguments
    ///  - second_operand: The previous result
    /// # Return
    /// A `Result` having a the arithmetic result of the operation if valid, `OperationError` otherwise
    pub fn apply_result(&self, second_operand: usize) -> Result<usize, OperationError> {
        trace!("{:?} {}", self, second_operand);
        match self {
            Self::Add(first_operand) => first_operand
                .checked_add(second_operand)
                .ok_or(OperationError::OverflowError),
            Self::Sub(first_operand) => first_operand
                .checked_sub(second_operand)
                .ok_or(OperationError::OverflowError),
            Self::Mul(first_operand) => first_operand
                .checked_mul(second_operand)
                .ok_or(OperationError::OverflowError),
            Self::Div(first_operand) => first_operand
                .checked_div(second_operand)
                .ok_or(OperationError::OverflowError),
        }
    }
}
