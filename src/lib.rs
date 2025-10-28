pub mod definitions;

use linguist_types::{HeuristicRule, Language};
use regex::Regex;
use std::path::Path;

/// Detects programming language(s) by file extension.
///
/// Returns a vector of tuples containing the language name and Language definition.
/// If multiple languages share the same extension, all are returned.
/// Use heuristics to disambiguate when multiple languages are returned.
///
/// # Arguments
///
/// * `filename` - Path or filename to check
///
/// # Returns
///
/// A vector of (name, language) tuples. Empty if no matching language is found.
///
/// # Examples
///
/// ```
/// use linguist::detect_language_by_extension;
///
/// // Unambiguous extension
/// let langs = detect_language_by_extension("script.py");
/// assert_eq!(langs.len(), 1);
/// assert_eq!(langs[0].0, "Python");
///
/// // Some extensions are ambiguous
/// let langs = detect_language_by_extension("header.h");
/// assert!(langs.iter().any(|(name, _)| *name == "C"));
/// assert!(langs.iter().any(|(name, _)| *name == "C++"));
/// ```
pub fn detect_language_by_extension<P: AsRef<Path>>(
    filename: P,
) -> Vec<(&'static str, &'static Language)> {
    let path = filename.as_ref();

    // Get the full filename
    let filename_str = path.file_name().and_then(|s| s.to_str()).unwrap_or("");

    if filename_str.is_empty() {
        return Vec::new();
    }

    // Search through all languages for matching extensions
    let mut matching_languages = Vec::new();

    for (lang_name, lang_def) in definitions::LANGUAGES.iter() {
        if let Some(ref extensions) = lang_def.extensions {
            // Check if the filename ends with any of this language's extensions
            // This handles both simple extensions (.rs) and compound ones (.rs.in, .blade.php)
            for ext in extensions {
                if filename_str.ends_with(ext) {
                    matching_languages.push((lang_name.as_str(), lang_def));
                    break; // Don't add the same language multiple times
                }
            }
        }
    }

    matching_languages
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
/// A vector of (name, language) tuples. Empty if no matching language is found.
///
/// # Examples
///
/// ```
/// use linguist::detect_language_by_filename;
///
/// let langs = detect_language_by_filename("Makefile");
/// assert_eq!(langs.len(), 1);
/// assert_eq!(langs[0].0, "Makefile");
///
/// let langs = detect_language_by_filename(".gitignore");
/// assert_eq!(langs.len(), 1);
/// assert_eq!(langs[0].0, "Ignore List");
/// ```
pub fn detect_language_by_filename<P: AsRef<Path>>(
    filename: P,
) -> Vec<(&'static str, &'static Language)> {
    let path = filename.as_ref();

    // Get the full filename (just the filename, not the path)
    let filename_str = path.file_name().and_then(|s| s.to_str()).unwrap_or("");

    if filename_str.is_empty() {
        return Vec::new();
    }

    // Search through all languages for matching filenames
    let mut matching_languages = Vec::new();

    for (lang_name, lang_def) in definitions::LANGUAGES.iter() {
        if let Some(ref filenames) = lang_def.filenames {
            // Check for exact filename match
            if filenames.iter().any(|f| f == filename_str) {
                matching_languages.push((lang_name.as_str(), lang_def));
            }
        }
    }

    matching_languages
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
/// An Option containing the (name, language) tuple if a match is found, None otherwise.
///
/// # Examples
///
/// ```
/// use linguist::disambiguate;
///
/// let content = "#!/usr/bin/env ruby\nputs 'Hello'";
/// let result = disambiguate("script.rb", content);
/// // Will return Some(("Ruby", &Language {...}))
/// ```
pub fn disambiguate<P: AsRef<Path>>(
    filename: P,
    file_contents: &str,
) -> Option<Vec<(&'static str, &'static Language)>> {
    let path = filename.as_ref();

    // Extract the extension
    let filename_str = path.file_name().and_then(|s| s.to_str()).unwrap_or("");

    if filename_str.is_empty() {
        return None;
    }

    // Find the extension(s) - we need to check all possible extensions
    // e.g., for "file.d.ts", we want to check both ".d.ts" and ".ts"
    let mut extensions = Vec::new();

    // First try to get the full extension path (for compound extensions)
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
    for disambiguation in &definitions::HEURISTICS.disambiguations {
        for ext in &extensions {
            if disambiguation.extensions.iter().any(|e| e == ext) {
                // Try each rule in order
                for rule in &disambiguation.rules {
                    if evaluate_rule(rule, file_contents) {
                        // Extract the language name(s) from the rule
                        if let Some(ref lang_names) = rule.language {
                            let mut matching_languages = Vec::new();
                            for lang_name in lang_names {
                                // Find the language definition by name
                                if let Some(lang_def) = definitions::LANGUAGES.get(lang_name) {
                                    matching_languages.push((lang_name.as_str(), lang_def));
                                }
                            }
                            return Some(matching_languages);
                        }
                    }
                }

                // We found the right disambiguation block but no rule matched
                return None;
            }
        }
    }

    None
}

/// Helper function to evaluate a single heuristic rule against file contents
fn evaluate_rule(rule: &HeuristicRule, file_contents: &str) -> bool {
    // If there's an 'and' clause, all sub-rules must match
    if let Some(ref and_rules) = rule.and {
        return and_rules.iter().all(|r| evaluate_rule(r, file_contents));
    }

    // Check named_pattern first
    if let Some(ref named_pattern) = rule.named_pattern {
        if let Some(pattern) = definitions::HEURISTICS.named_patterns.get(named_pattern) {
            if !matches_pattern(pattern, file_contents) {
                return false;
            }
        } else {
            // Named pattern doesn't exist - treat as non-match
            return false;
        }
    }

    // Check positive pattern
    if let Some(ref pattern) = rule.pattern {
        if !matches_pattern(pattern, file_contents) {
            return false;
        }
    }

    // Check negative pattern (must NOT match)
    if let Some(ref neg_pattern) = rule.negative_pattern {
        if matches_pattern(neg_pattern, file_contents) {
            return false;
        }
    }

    // If we get here, all conditions passed (or there were no conditions = always match)
    true
}

/// Helper function to check if any pattern in a list matches the content
fn matches_pattern(patterns: &[String], content: &str) -> bool {
    patterns.iter().any(|p| {
        if let Ok(regex) = Regex::new(p) {
            regex.is_match(content)
        } else {
            false
        }
    })
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
/// `true` if the file matches any vendor pattern, `false` otherwise.
///
/// # Examples
///
/// ```
/// use linguist::is_vendored;
///
/// assert!(is_vendored("node_modules/react/index.js"));
/// assert!(is_vendored("vendor/bundle/gems/rails.rb"));
/// assert!(!is_vendored("src/main.rs"));
/// ```
pub fn is_vendored<P: AsRef<Path>>(filename: P) -> bool {
    let path = filename.as_ref();
    let path_str = path.to_str().unwrap_or("");

    if path_str.is_empty() {
        return false;
    }

    // Check if the path matches any vendor pattern
    definitions::VENDOR.iter().any(|pattern| {
        if let Ok(regex) = Regex::new(pattern) {
            regex.is_match(path_str)
        } else {
            false
        }
    })
}
