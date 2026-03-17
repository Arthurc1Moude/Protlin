use std::fmt;

#[derive(Debug, Clone)]
pub enum ProtlinError {
    LexerError(String),
    ParserError(String),
    RuntimeError(String),
    TypeError(String),
    DivisionByZero,
    UndefinedVariable(String),
    UndefinedFunction(String),
    InvalidOperation(String),
    InvalidArgument(String),
    IndexOutOfBounds,
    InvalidType(String),
    BreakOutsideLoop,
    ContinueOutsideLoop,
    ReturnOutsideFunction,
    InvalidPattern(String),
    ModuleNotFound(String),
    ImportError(String),
}

impl fmt::Display for ProtlinError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProtlinError::LexerError(msg) => write!(f, "Lexer error: {}", msg),
            ProtlinError::ParserError(msg) => write!(f, "Parser error: {}", msg),
            ProtlinError::RuntimeError(msg) => write!(f, "Runtime error: {}", msg),
            ProtlinError::TypeError(msg) => write!(f, "Type error: {}", msg),
            ProtlinError::DivisionByZero => write!(f, "Division by zero"),
            ProtlinError::UndefinedVariable(name) => write!(f, "Undefined variable: {}", name),
            ProtlinError::UndefinedFunction(name) => write!(f, "Undefined function: {}", name),
            ProtlinError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
            ProtlinError::InvalidArgument(msg) => write!(f, "Invalid argument: {}", msg),
            ProtlinError::IndexOutOfBounds => write!(f, "Index out of bounds"),
            ProtlinError::InvalidType(msg) => write!(f, "Invalid type: {}", msg),
            ProtlinError::BreakOutsideLoop => write!(f, "Break statement outside loop"),
            ProtlinError::ContinueOutsideLoop => write!(f, "Continue statement outside loop"),
            ProtlinError::ReturnOutsideFunction => write!(f, "Return statement outside function"),
            ProtlinError::InvalidPattern(msg) => write!(f, "Invalid pattern: {}", msg),
            ProtlinError::ModuleNotFound(name) => write!(f, "Module not found: {}", name),
            ProtlinError::ImportError(msg) => write!(f, "Import error: {}", msg),
        }
    }
}

impl std::error::Error for ProtlinError {}
