use std::fmt;

/// Errors that can occur during language detection
#[derive(Debug, Clone, PartialEq)]
pub enum LinguistError {
    /// The provided path is invalid for whatever reason
    InvalidPath(String),

    /// A regex pattern in the heuristics is malformed
    InvalidRegex { pattern: String, error: String },

    /// A named pattern referenced in heuristics doesn't exist
    MissingNamedPattern(String),
}

impl fmt::Display for LinguistError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LinguistError::InvalidPath(path) => {
                write!(f, "Invalid path: {}", path)
            }
            LinguistError::InvalidRegex { pattern, error } => {
                write!(f, "Invalid regex pattern '{}': {}", pattern, error)
            }
            LinguistError::MissingNamedPattern(name) => {
                write!(f, "Named pattern '{}' not found in heuristics", name)
            }
        }
    }
}

impl std::error::Error for LinguistError {}
