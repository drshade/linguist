pub mod definitions;
pub mod error;
pub(crate) mod indexed;
pub mod utils;

use linguist_types::{HeuristicRule, Language};
use std::path::Path;

pub use error::LinguistError;

/// Type alias for Results in this crate
pub type Result<T> = std::result::Result<T, LinguistError>;

/// Represents a detected programming language.
///
/// Contains both the language name and full language definition with metadata.
#[derive(Debug, Clone)]
pub struct DetectedLanguage {
    /// The name of the detected language (e.g., "Python", "Rust")
    pub name: &'static str,

    /// The full language definition with metadata (type, color, extensions, etc.)
    pub definition: &'static Language,
}

/// Detects programming language(s) by file extension.
///
/// Returns a Result containing either:
/// - Ok(Vec<DetectedLanguage>) with matching languages (empty vec if no matches)
/// - Err(LinguistError) on error
///
/// # Arguments
///
/// * `filename` - Path or filename to check
///
/// # Returns
///
/// A Result with a vector of DetectedLanguage. Empty if no matching language is found.
///
/// # Examples
///
/// ```
/// use linguist::detect_language_by_extension;
///
/// // Unambiguous extension
/// let langs = detect_language_by_extension("script.py")?;
/// assert_eq!(langs.len(), 1);
/// assert_eq!(langs[0].name, "Python");
///
/// // Some extensions are ambiguous
/// let langs = detect_language_by_extension("header.h")?;
/// assert!(langs.iter().any(|lang| lang.name == "C"));
/// assert!(langs.iter().any(|lang| lang.name == "C++"));
/// # Ok::<(), linguist::LinguistError>(())
/// ```
pub fn detect_language_by_extension<P: AsRef<Path>>(filepath: P) -> Result<Vec<DetectedLanguage>> {
    // Get just the filename
    //
    let filename_str = utils::get_filename_from_path(filepath.as_ref())?;

    // Use the extension index for O(1) lookup per extension
    let mut matching_languages = Vec::new();
    let mut seen_languages = std::collections::HashSet::new();

    for extension in &utils::extract_extensions(filename_str) {
        if let Some(language_names) = indexed::LANGUAGES_BY_EXTENSION.get(extension) {
            for lang_name in language_names {
                // Avoid adding the same language multiple times (HashSet already prevents
                // duplicates within an extension, but we need to check across extensions)
                if seen_languages.insert(lang_name) {
                    if let Some(lang_def) = definitions::LANGUAGES.get(lang_name) {
                        matching_languages.push(DetectedLanguage {
                            name: lang_name.as_str(),
                            definition: lang_def,
                        });
                    }
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
/// A Result with a vector of DetectedLanguage. Empty if no matching language is found.
///
/// # Examples
///
/// ```
/// use linguist::detect_language_by_filename;
///
/// let langs = detect_language_by_filename("Makefile")?;
/// assert_eq!(langs.len(), 1);
/// assert_eq!(langs[0].name, "Makefile");
///
/// let langs = detect_language_by_filename(".gitignore")?;
/// assert_eq!(langs.len(), 1);
/// assert_eq!(langs[0].name, "Ignore List");
/// # Ok::<(), linguist::LinguistError>(())
/// ```
pub fn detect_language_by_filename<P: AsRef<Path>>(filepath: P) -> Result<Vec<DetectedLanguage>> {
    // Get just the filename
    //
    let filename_str = utils::get_filename_from_path(filepath.as_ref())?;

    // Use the filename index for O(1) lookup
    //
    let mut matching_languages = Vec::new();
    if let Some(language_names) = indexed::LANGUAGES_BY_FILENAME.get(filename_str) {
        for lang_name in language_names {
            if let Some(lang_def) = definitions::LANGUAGES.get(lang_name) {
                matching_languages.push(DetectedLanguage {
                    name: lang_name.as_str(),
                    definition: lang_def,
                });
            }
        }
    }

    Ok(matching_languages)
}

/// Disambiguates between multiple languages for a file using heuristic rules.
///
/// When multiple languages share the same file extension, this function uses
/// regex-based heuristics to determine the most likely language based on file content.
/// Uses a pre-built index for O(1) extension lookup performance.
///
/// # Arguments
///
/// * `filename` - Path or filename to check (used to extract extension)
/// * `file_contents` - The contents of the file to analyze
///
/// # Returns
///
/// A Result containing a vector of DetectedLanguage if a match is found (empty vec if
/// no heuristic rules match), or an error if the path is invalid or regex patterns are malformed.
///
/// # Examples
///
/// ```
/// use linguist::disambiguate;
///
/// let content = "#!/usr/bin/env ruby\nputs 'Hello'";
/// let result = disambiguate("script.rb", content)?;
/// // Will return Ok(vec![DetectedLanguage { name: "Ruby", ... }])
/// # Ok::<(), linguist::LinguistError>(())
/// ```
pub fn disambiguate<P: AsRef<Path>>(
    filepath: P,
    file_contents: &str,
) -> Result<Vec<DetectedLanguage>> {
    // Get just the filename
    //
    let filename_str = utils::get_filename_from_path(filepath.as_ref())?;

    // Look up disambiguations using the index for O(1) performance
    for extension in &utils::extract_extensions(filename_str) {
        if let Some(disambiguations) = indexed::DISAMBIGUATIONS_BY_EXTENSION.get(extension) {
            // Try each disambiguation that applies to this extension
            for disambiguation in disambiguations {
                // Try each rule in this disambiguation
                for rule in &disambiguation.rules {
                    if evaluate_rule(rule, file_contents)? {
                        if let Some(ref lang_names) = rule.language {
                            let mut matching_languages = Vec::new();
                            for lang_name in lang_names {
                                // If we have a hit - find the language definition by name in the
                                // LANGUAGES struct
                                if let Some(lang_def) = definitions::LANGUAGES.get(lang_name) {
                                    matching_languages.push(DetectedLanguage {
                                        name: lang_name.as_str(),
                                        definition: lang_def,
                                    });
                                }
                            }
                            return Ok(matching_languages);
                        }
                    }
                }
            }
        }
    }

    // No disambiguation rules matched - this is not an error,
    // just means the file doesn't need disambiguation or no rules applied
    Ok(vec![])
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
                if !utils::matches_pattern(pattern, file_contents)? {
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
        if !utils::matches_pattern(pattern, file_contents)? {
            return Ok(false);
        }
    }

    // Check negative pattern
    //
    if let Some(ref neg_pattern) = rule.negative_pattern {
        if utils::matches_pattern(neg_pattern, file_contents)? {
            return Ok(false);
        }
    }

    // Otherwise it's a match!
    //
    Ok(true)
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
pub fn is_vendored<P: AsRef<Path>>(filepath: P) -> Result<bool> {
    let path = filepath.as_ref();
    let path_str = path
        .to_str()
        .ok_or_else(|| LinguistError::InvalidPath(format!("{:?}", path)))?;

    // Check if the path matches any precompiled vendor pattern
    //
    Ok(indexed::VENDOR_PATTERNS
        .iter()
        .any(|regex| regex.is_match(path_str)))
}
