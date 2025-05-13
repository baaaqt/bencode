use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum ErrorCode {
    UnexpectedCharacter,
    IntegerParseError,
    StringParseError,
    InvalidList,
    InvalidDictionary,
}

impl ErrorCode {
    pub fn to_string(&self) -> String {
        match self {
            ErrorCode::UnexpectedCharacter => "Unexpected character".to_string(),
            ErrorCode::StringParseError => "String parsing error".to_string(),
            ErrorCode::InvalidList => "Invalid list".to_string(),
            ErrorCode::InvalidDictionary => "Invalid dictionary".to_string(),
            ErrorCode::IntegerParseError => "Integer parsing error".to_string(),
        }
    }
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

/// Custom error type for Bencode parsing errors.
/// This struct contains an error code and the position in the input data where the error occurred.
#[derive(Debug, Clone)]
pub struct Error {
    code: ErrorCode,
    position: usize,
    message: String,
}

impl Error {
    pub(crate) fn new(code: ErrorCode, position: usize, message: String) -> Self {
        Error {
            code,
            position,
            message,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{}. Position: {}. Message: {}",
            self.code.to_string(),
            self.position,
            self.message,
        )
    }

    pub fn code(&self) -> ErrorCode {
        self.code
    }

    pub fn position(&self) -> usize {
        self.position
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl std::error::Error for Error {}
