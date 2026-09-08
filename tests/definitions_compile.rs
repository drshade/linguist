//! Every regex in the vendored definitions must compile with `fancy-regex`.
//!
//! Upstream github-linguist writes its heuristics for Ruby's Oniguruma engine. The library
//! compiles heuristic patterns lazily, only when a file with that extension is actually
//! disambiguated, and silently drops vendor patterns that fail to compile. So without this
//! test an unsupported pattern would only surface at runtime, and only for files of the
//! affected extension — and only in the samples suite if upstream happens to ship a sample
//! for it. This test walks every pattern eagerly so a bad upstream sync fails CI outright.

use linguist::definitions::{HEURISTICS, VENDOR};
use linguist_types::HeuristicRule;

/// Collect every pattern a heuristic rule (recursively, through `and`) can evaluate.
fn rule_patterns<'a>(rule: &'a HeuristicRule, out: &mut Vec<(String, &'a str)>, location: &str) {
    for (field, patterns) in [
        ("pattern", &rule.pattern),
        ("negative_pattern", &rule.negative_pattern),
    ] {
        for pattern in patterns.iter().flatten() {
            out.push((format!("{location}/{field}"), pattern));
        }
    }
    for (i, sub_rule) in rule.and.iter().flatten().enumerate() {
        rule_patterns(sub_rule, out, &format!("{location}/and[{i}]"));
    }
}

#[test]
fn every_heuristic_pattern_compiles() {
    let mut patterns = Vec::new();

    for (name, named) in &HEURISTICS.named_patterns {
        for pattern in named {
            patterns.push((format!("named_patterns/{name}"), pattern.as_str()));
        }
    }
    for disambiguation in &HEURISTICS.disambiguations {
        let extensions = disambiguation.extensions.join(",");
        for rule in &disambiguation.rules {
            let language = rule.language.as_deref().unwrap_or_default().join(",");
            rule_patterns(
                rule,
                &mut patterns,
                &format!("disambiguations[{extensions}]/rules[{language}]"),
            );
        }
    }
    assert!(
        !patterns.is_empty(),
        "no heuristic patterns found — build script broken?"
    );

    // Compile through the library's own path so this exercises exactly what production
    // compiles (including the multi-line prefix it adds), not a re-implementation of it.
    let failures: Vec<String> = patterns
        .iter()
        .filter_map(|(location, pattern)| {
            linguist::utils::matches_pattern(std::slice::from_ref(&pattern.to_string()), "")
                .err()
                .map(|e| format!("  {location}\n    pattern: {pattern:?}\n    error: {e}"))
        })
        .collect();

    assert!(
        failures.is_empty(),
        "{} of {} heuristic patterns failed to compile with fancy-regex \
         (fix upstream — see MAINTAINING.md):\n{}",
        failures.len(),
        patterns.len(),
        failures.join("\n")
    );
}

#[test]
fn every_vendor_pattern_compiles() {
    assert!(
        !VENDOR.is_empty(),
        "no vendor patterns found — build script broken?"
    );

    // `is_vendored` compiles these with a plain `Regex::new` and drops any that fail with
    // only a warning on stderr, so mirror that call exactly and make failure loud.
    let failures: Vec<String> = VENDOR
        .iter()
        .filter_map(|pattern| {
            fancy_regex::Regex::new(pattern)
                .err()
                .map(|e| format!("  pattern: {pattern:?}\n    error: {e}"))
        })
        .collect();

    assert!(
        failures.is_empty(),
        "{} of {} vendor patterns failed to compile with fancy-regex:\n{}",
        failures.len(),
        VENDOR.len(),
        failures.join("\n")
    );
}
