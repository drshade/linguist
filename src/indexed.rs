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

pub type Alias = String;

/// Languages keyed by lowercased alias, mirroring upstream's `Language.find_by_alias`: each
/// language's default alias (its name, lowercased, whitespace replaced by `-`) plus every
/// entry in its `aliases` list.
pub static LANGUAGES_BY_ALIAS: LazyLock<HashMap<Alias, LanguageName>> = LazyLock::new(|| {
    let mut index = HashMap::new();

    for (lang_name, lang_def) in definitions::LANGUAGES.iter() {
        let default_alias = lang_name.to_lowercase().replace(char::is_whitespace, "-");
        index.insert(default_alias, lang_name.clone());
        for alias in lang_def.aliases.iter().flatten() {
            index.insert(alias.to_lowercase(), lang_name.clone());
        }
    }

    index
});

/// Emacs and Vim modeline patterns, in the order they are tried. Each captures the mode or
/// filetype name in group 1. These are collapsed forms of `EMACS_MODELINE` and
/// `VIM_MODELINE` in upstream's `lib/linguist/strategy/modeline.rb` (which documents them
/// piece by piece). Ruby's `^`/`$` always anchor to lines, hence `(?m)`; Ruby's `(?-m)` is
/// the default `.` behaviour here.
pub static MODELINE_REGEXES: LazyLock<[Regex; 2]> = LazyLock::new(|| {
    const EMACS: &str = r"(?i)-\*-(?:[ \t]*(?=[^:;\s]+[ \t]*-\*-)|(?:.*?[ \t;]|(?<=-\*-))[ \t]*mode[ \t]*:[ \t]*)([^:;\s]+)(?=[ \t;]|(?<![-*])-\*-).*?-\*-";
    const VIM: &str = r"(?m)(?:(?:^|[ \t])(?:vi|Vi(?=m))(?:m[<=>]?[0-9]+|m)?|[ \t]ex)(?=:(?=[ \t]*set?[ \t][^\r\n:]+:)|:(?![ \t]*set?[ \t]))(?:(?:[ \t]*:[ \t]*|[ \t])\w*(?:[ \t]*=(?:[^\\\s]|\\.)*)?)*[ \t:](?:filetype|ft|syntax)[ \t]*=(\w+)(?=$|\s|:)";
    [EMACS, VIM].map(|pattern| Regex::new(pattern).expect("built-in modeline regex compiles"))
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
