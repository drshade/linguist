use crate::error::LinguistError;
use regex::Regex;
use std::path::Path;

/// Type alias for Results in this crate
pub type Result<T> = std::result::Result<T, LinguistError>;

/// Helper function to extract filename from a path.
///
/// This utility function extracts the filename portion from a path and converts it to a string.
/// It handles invalid paths and non-UTF8 filenames with appropriate error messages.
///
/// # Arguments
///
/// * `path` - A path reference to extract the filename from
///
/// # Returns
///
/// A Result containing the filename string or a LinguistError if the path is invalid.
///
/// # Examples
///
/// ```
/// use linguist::utils::get_filename_from_path;
/// use std::path::Path;
///
/// let result = get_filename_from_path(Path::new("src/main.rs"))?;
/// assert_eq!(result, "main.rs");
///
/// let result = get_filename_from_path(Path::new("Makefile"))?;
/// assert_eq!(result, "Makefile");
/// # Ok::<(), linguist::LinguistError>(())
/// ```
pub fn get_filename_from_path(path: &Path) -> Result<&str> {
    let filename_str = path
        .file_name()
        .ok_or(LinguistError::InvalidPath("Not a filename".to_string()))?
        .to_str()
        .ok_or_else(|| LinguistError::InvalidPath(format!("{:?}", path)))?;
    Ok(filename_str)
}

/// Extract all possible extensions from a filename.
///
/// Returns extensions from most specific to least specific.
/// For example, "file.a.b.c" returns [".c", ".b.c", ".a.b.c"]
///
/// # Examples
///
/// ```
/// use linguist::utils::extract_extensions;
///
/// assert_eq!(extract_extensions("test.py"), vec![".py".to_string()]);
/// assert_eq!(extract_extensions("file.d.ts"), vec![".ts".to_string(), ".d.ts".to_string()]);
/// assert_eq!(extract_extensions("complex.a.b.c"), vec![".c".to_string(), ".b.c".to_string(), ".a.b.c".to_string()]);
/// assert_eq!(extract_extensions("no-extension"), Vec::<String>::new());
/// ```
pub fn extract_extensions(filename: &str) -> Vec<String> {
    // Find all dot positions in the filename
    let dot_positions: Vec<usize> = filename
        .char_indices()
        .filter_map(|(i, c)| if c == '.' { Some(i) } else { None })
        .collect();

    // Generate extensions from shortest to longest (right to left)
    // For "file.d.ts": iterate [6, 4] to produce [".ts", ".d.ts"]
    dot_positions
        .iter()
        .rev()
        .map(|&pos| filename[pos..].to_string())
        .collect()
}

/// Helper function to check if any pattern in a list matches the content.
///
/// Returns true if any regex pattern in the list matches the content,
/// false if none match, or an error if any regex pattern is malformed.
///
/// # Examples
///
/// ```
/// use linguist::utils::matches_pattern;
///
/// let patterns = vec!["hello".to_string(), "world".to_string()];
/// assert!(matches_pattern(&patterns, "hello there").unwrap());
/// assert!(matches_pattern(&patterns, "world peace").unwrap());
/// assert!(!matches_pattern(&patterns, "goodbye").unwrap());
/// ```
pub fn matches_pattern(patterns: &[String], content: &str) -> Result<bool> {
    for pattern in patterns {
        let regex = Regex::new(pattern).map_err(|e| LinguistError::InvalidRegex {
            pattern: pattern.clone(),
            error: e.to_string(),
        })?;

        if regex.is_match(content) {
            return Ok(true);
        }
    }
    Ok(false)
}
