use std::fmt;

/// Errors that can occur during language detection operations
#[derive(Debug, Clone, PartialEq)]
pub enum LinguistError {
    /// The provided path contains invalid UTF-8 sequences
    InvalidPath(String),

    /// The provided path has no filename component (e.g., is root "/")
    NoFilename,

    /// A regex pattern in the heuristics is malformed
    InvalidRegex { pattern: String, error: String },

    /// A named pattern referenced in heuristics doesn't exist
    MissingNamedPattern(String),
}

impl fmt::Display for LinguistError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LinguistError::InvalidPath(path) => {
                write!(f, "Path contains invalid UTF-8: {}", path)
            }
            LinguistError::NoFilename => {
                write!(f, "Path has no filename component")
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

/// Type alias for Results in this crate
pub type Result<T> = std::result::Result<T, LinguistError>;
