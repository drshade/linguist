use crate::definitions;
use crate::error::{LinguistError, Result};
use fancy_regex::Regex;
use linguist_types::{Disambiguation, HeuristicRule};
use std::collections::{BTreeSet, HashMap};
use std::sync::{LazyLock, OnceLock};

//
// Indexed structures for faster lookups etc
//

pub type Filename = String;
pub type Extension = String;
pub type LanguageName = String;

pub static LANGUAGES_BY_FILENAME: LazyLock<HashMap<Filename, BTreeSet<LanguageName>>> =
    LazyLock::new(|| {
        // Process the LANGUAGES struct, building up the index
        //
        let mut index = HashMap::new();

        for (lang_name, lang_def) in definitions::LANGUAGES.iter() {
            if let Some(ref filenames) = lang_def.filenames {
                for filename in filenames {
                    index
                        .entry(filename.clone())
                        .or_insert_with(BTreeSet::new)
                        .insert(lang_name.clone());
                }
            }
        }

        index
    });

pub static LANGUAGES_BY_EXTENSION: LazyLock<HashMap<Extension, BTreeSet<LanguageName>>> =
    LazyLock::new(|| {
        // Process the LANGUAGES struct, building up the index
        //
        let mut index = HashMap::new();

        for (lang_name, lang_def) in definitions::LANGUAGES.iter() {
            if let Some(ref extensions) = lang_def.extensions {
                for extension in extensions {
                    index
                        .entry(extension.clone())
                        .or_insert_with(BTreeSet::new)
                        .insert(lang_name.clone());
                }
            }
        }

        index
    });

pub static DISAMBIGUATIONS_BY_EXTENSION: LazyLock<HashMap<Extension, Vec<Disambiguation>>> =
    LazyLock::new(|| {
        // Process the HEURISTICS struct, building up the index
        //
        let mut index = HashMap::new();

        for disambiguation in &definitions::HEURISTICS.disambiguations {
            for extension in &disambiguation.extensions {
                index
                    .entry(extension.clone())
                    .or_insert_with(Vec::new)
                    .push(disambiguation.clone());
            }
        }

        index
    });

pub static VENDOR_PATTERNS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    // Precompile all vendor regex patterns
    //
    definitions::VENDOR
        .iter()
        .filter_map(|pattern| match Regex::new(pattern) {
            Ok(regex) => Some(regex),
            Err(e) => {
                // Should we rather abort? I dunno...
                // For now we will report to stderr and ignore the pattern
                //
                eprintln!("Warning: Failed to compile vendor pattern '{pattern}': {e}");
                None
            }
        })
        .collect()
});

/// Compile a heuristic pattern. Heuristic patterns are applied in multi-line mode so that
/// `^`/`$` anchor to line boundaries, matching upstream linguist's behaviour.
pub fn compile_heuristic_pattern(pattern: &str) -> Result<Regex> {
    Regex::new(&format!("(?m){pattern}")).map_err(|e| LinguistError::InvalidRegex {
        pattern: pattern.to_string(),
        error: e.to_string(),
    })
}

/// Every regex pattern referenced by `HEURISTICS` (named patterns plus every rule's positive
/// and negative patterns, including nested `and` rules), keyed by its source text.
///
/// Each pattern is compiled at most once, on first use, and the compiled regex is reused for
/// every subsequent file. Compilation is deferred per pattern (rather than done eagerly when
/// the table is built) so a process that only ever looks at a handful of extensions doesn't
/// pay to compile the whole table up front.
pub static HEURISTIC_REGEXES: LazyLock<HashMap<&'static str, OnceLock<Result<Regex>>>> =
    LazyLock::new(|| {
        let mut index = HashMap::new();

        fn collect_rule<'a>(
            rule: &'a HeuristicRule,
            index: &mut HashMap<&'a str, OnceLock<Result<Regex>>>,
        ) {
            for patterns in [&rule.pattern, &rule.negative_pattern]
                .into_iter()
                .flatten()
            {
                for pattern in patterns {
                    index.entry(pattern.as_str()).or_default();
                }
            }
            for sub_rule in rule.and.iter().flatten() {
                collect_rule(sub_rule, index);
            }
        }

        for patterns in definitions::HEURISTICS.named_patterns.values() {
            for pattern in patterns {
                index.entry(pattern.as_str()).or_default();
            }
        }
        for disambiguation in &definitions::HEURISTICS.disambiguations {
            for rule in &disambiguation.rules {
                collect_rule(rule, &mut index);
            }
        }

        index
    });
