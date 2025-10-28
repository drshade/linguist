pub mod definitions;
pub mod error;

use linguist_types::{HeuristicRule, Language};
use regex::Regex;
use std::path::Path;

pub use error::LinguistError;

/// Type alias for Results in this crate
pub type Result<T> = std::result::Result<T, LinguistError>;

/// Detects programming language(s) by file extension.
///
/// Returns a Result containing either:
/// - Ok(Vec<..>) with matching languages (empty vec if no matches)
/// - Err(LinguistError) on error
///
/// # Arguments
///
/// * `filename` - Path or filename to check
///
/// # Returns
///
/// A Result with a vector of (name, language) tuples. Empty if no matching language is found.
///
/// # Examples
///
/// ```
/// use linguist::detect_language_by_extension;
///
/// // Unambiguous extension
/// let langs = detect_language_by_extension("script.py")?;
/// assert_eq!(langs.len(), 1);
/// assert_eq!(langs[0].0, "Python");
///
/// // Some extensions are ambiguous
/// let langs = detect_language_by_extension("header.h")?;
/// assert!(langs.iter().any(|(name, _)| *name == "C"));
/// assert!(langs.iter().any(|(name, _)| *name == "C++"));
/// # Ok::<(), linguist::LinguistError>(())
/// ```
pub fn detect_language_by_extension<P: AsRef<Path>>(
    filename: P,
) -> Result<Vec<(&'static String, &'static Language)>> {
    let path = filename.as_ref();

    // Get the filename
    //
    let filename_str = path
        .file_name()
        .ok_or(LinguistError::InvalidPath("Not a filename".to_string()))?
        .to_str()
        .ok_or_else(|| LinguistError::InvalidPath(format!("{:?}", path)))?;

    // Search through all languages for matching extensions
    //
    let mut matching_languages = Vec::new();

    // TODO: Super slow. We need to index here.
    //
    for entry @ (_, lang) in definitions::LANGUAGES.iter() {
        if let Some(ref extensions) = lang.extensions {
            // Check if the filename ends with any of this language's extensions
            //
            for ext in extensions {
                if filename_str.ends_with(ext) {
                    matching_languages.push(entry);
                    break; // Don't add the same language multiple times
                }
            }
        }
    }

    Ok(matching_languages)
}

/// Detects programming language(s) by exact filename match.
///
/// Returns languages that have the exact filename in their `filenames` list.
/// This is used for special files like "Makefile", ".gitignore", "Dockerfile", etc.
///
/// # Arguments
///
/// * `filename` - Path or filename to check
///
/// # Returns
///
/// A Result with a vector of (name, language) tuples. Empty if no matching language is found.
///
/// # Examples
///
/// ```
/// use linguist::detect_language_by_filename;
///
/// let langs = detect_language_by_filename("Makefile")?;
/// assert_eq!(langs.len(), 1);
/// assert_eq!(langs[0].0, "Makefile");
///
/// let langs = detect_language_by_filename(".gitignore")?;
/// assert_eq!(langs.len(), 1);
/// assert_eq!(langs[0].0, "Ignore List");
/// # Ok::<(), linguist::LinguistError>(())
/// ```
pub fn detect_language_by_filename<P: AsRef<Path>>(
    filename: P,
) -> Result<Vec<(&'static String, &'static Language)>> {
    let path = filename.as_ref();

    // Get the filename
    //
    let filename_str = path
        .file_name()
        .ok_or(LinguistError::InvalidPath("Not a filename".to_string()))?
        .to_str()
        .ok_or_else(|| LinguistError::InvalidPath(format!("{:?}", path)))?;

    // Search through all languages for matching filenames
    //
    let mut matching_languages = Vec::new();

    // TODO: Super slow. Need to index.
    //
    for entry @ (_, lang) in definitions::LANGUAGES.iter() {
        if let Some(ref filenames) = lang.filenames {
            if filenames.iter().any(|f| f == filename_str) {
                matching_languages.push(entry);
            }
        }
    }

    Ok(matching_languages)
}

/// Disambiguates between multiple languages for a file using heuristic rules.
///
/// When multiple languages share the same file extension, this function uses
/// regex-based heuristics to determine the most likely language based on file content.
///
/// # Arguments
///
/// * `filename` - Path or filename to check (used to extract extension)
/// * `file_contents` - The contents of the file to analyze
///
/// # Returns
///
/// A Result containing an Option with (name, language) tuples if a match is found, None if
/// no heuristic rules match, or an error if the path is invalid or regex patterns are malformed.
///
/// # Examples
///
/// ```
/// use linguist::disambiguate;
///
/// let content = "#!/usr/bin/env ruby\nputs 'Hello'";
/// let result = disambiguate("script.rb", content)?;
/// // Will return Ok(Some(vec![("Ruby", &Language {...})]))
/// # Ok::<(), linguist::LinguistError>(())
/// ```
pub fn disambiguate<P: AsRef<Path>>(
    filename: P,
    file_contents: &str,
) -> Result<Option<Vec<(&'static String, &'static Language)>>> {
    let path = filename.as_ref();

    // Extract the filename
    //
    let filename_str = path
        .file_name()
        .ok_or(LinguistError::InvalidPath("Not a filename".to_string()))?
        .to_str()
        .ok_or_else(|| LinguistError::InvalidPath(format!("{:?}", path)))?;

    // Find the extension(s) - we need to check all possible extensions
    // e.g., for "file.d.ts", we want to check both ".d.ts" and ".ts"
    // TODO: I'm not sure this is ideal...
    let mut extensions = Vec::new();
    if let Some(dot_pos) = filename_str.find('.') {
        extensions.push(&filename_str[dot_pos..]);

        // Also add the simple extension
        if let Some(last_dot_pos) = filename_str.rfind('.') {
            if last_dot_pos != dot_pos {
                extensions.push(&filename_str[last_dot_pos..]);
            }
        }
    }

    // Find the disambiguation that matches our extension(s)
    //
    for disambiguation in &definitions::HEURISTICS.disambiguations {
        for ext in &extensions {
            if disambiguation.extensions.iter().any(|e| e == ext) {
                // Try each rule
                //
                for rule in &disambiguation.rules {
                    if evaluate_rule(rule, file_contents)? {
                        if let Some(ref lang_names) = rule.language {
                            let mut matching_languages = Vec::new();
                            for lang_name in lang_names {
                                // If we have a hit - find the language definition by name in the
                                // LANGUAGES struct
                                //
                                if let Some(lang_def) = definitions::LANGUAGES.get(lang_name) {
                                    matching_languages.push((lang_name, lang_def));
                                }
                            }
                            return Ok(Some(matching_languages));
                        }
                    }
                }

                // We found the right disambiguation block but no rule matched
                //
                return Ok(None);
            }
        }
    }

    // No disambiguation rules found for this extension - this is not an error,
    // just means the file doesn't need disambiguation
    //
    Ok(None)
}

/// Helper function to evaluate a single heuristic rule against file contents
///
fn evaluate_rule(rule: &HeuristicRule, file_contents: &str) -> Result<bool> {
    // If there's an 'and' clause, all sub-rules must match
    //
    if let Some(ref and_rules) = rule.and {
        for sub_rule in and_rules {
            if !evaluate_rule(sub_rule, file_contents)? {
                return Ok(false);
            }
        }
        return Ok(true);
    }

    // Check named_pattern first
    //
    if let Some(ref named_pattern) = rule.named_pattern {
        match definitions::HEURISTICS.named_patterns.get(named_pattern) {
            Some(pattern) => {
                if !matches_pattern(pattern, file_contents)? {
                    return Ok(false);
                }
            }
            None => {
                return Err(LinguistError::MissingNamedPattern(named_pattern.clone()));
            }
        }
    }

    // Check positive pattern
    //
    if let Some(ref pattern) = rule.pattern {
        if !matches_pattern(pattern, file_contents)? {
            return Ok(false);
        }
    }

    // Check negative pattern
    //
    if let Some(ref neg_pattern) = rule.negative_pattern {
        if matches_pattern(neg_pattern, file_contents)? {
            return Ok(false);
        }
    }

    // Otherwise it's a match!
    //
    Ok(true)
}

/// Helper function to check if any pattern in a list matches the content
///
fn matches_pattern(patterns: &[String], content: &str) -> Result<bool> {
    for pattern in patterns {
        match Regex::new(pattern) {
            Ok(regex) => {
                if regex.is_match(content) {
                    return Ok(true);
                }
            }
            Err(e) => {
                return Err(LinguistError::InvalidRegex {
                    pattern: pattern.clone(),
                    error: e.to_string(),
                });
            }
        }
    }
    Ok(false)
}

/// Checks if a file is a vendored/third-party file that should typically be excluded from statistics.
///
/// Vendored files are dependencies, libraries, or generated code that are not part of the
/// main project codebase. Examples include node_modules, vendor directories, minified files, etc.
///
/// # Arguments
///
/// * `filename` - Path or filename to check
///
/// # Returns
///
/// A Result containing `true` if the file matches any vendor pattern, `false` otherwise.
/// Returns an error if the path is invalid or regex patterns are malformed.
///
/// # Examples
///
/// ```
/// use linguist::is_vendored;
///
/// assert!(is_vendored("node_modules/react/index.js")?);
/// assert!(is_vendored("vendor/bundle/gems/rails.rb")?);
/// assert!(!is_vendored("src/main.rs")?);
/// # Ok::<(), linguist::LinguistError>(())
/// ```
pub fn is_vendored<P: AsRef<Path>>(filename: P) -> Result<bool> {
    let path = filename.as_ref();
    let path_str = path
        .to_str()
        .ok_or_else(|| LinguistError::InvalidPath(format!("{:?}", path)))?;

    // Check if the path matches any vendor pattern
    //
    matches_pattern(&definitions::VENDOR, path_str)
}
