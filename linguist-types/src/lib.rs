use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

/// Represents the type of a language as defined in languages.yml
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LanguageType {
    Data,
    Programming,
    Markup,
    Prose,
}

/// Represents a single language definition from languages.yml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Language {
    /// Type of language: data, programming, markup, or prose (required)
    #[serde(rename = "type")]
    pub language_type: LanguageType,

    /// Ace editor mode used for syntax highlighting (required)
    pub ace_mode: String,

    /// TextMate scope for the language (required)
    pub tm_scope: String,

    /// Unique identifier used internally by GitHub (required)
    pub language_id: i64,

    /// List of associated file extensions
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<String>>,

    /// List of associated filenames
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filenames: Option<Vec<String>>,

    /// Additional aliases for the language
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,

    /// List of programs that execute the language
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interpreters: Option<Vec<String>>,

    /// CSS color code representing the language (format: "#RRGGBB")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    /// CodeMirror 5 mode for editing
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub codemirror_mode: Option<String>,

    /// MIME media-type used by CodeMirror 5
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub codemirror_mime_type: Option<String>,

    /// Name of the parent language (for grouping statistics)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,

    /// Filesystem-safe name for the language
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fs_name: Option<String>,

    /// Enable soft line-wrapping
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wrap: Option<bool>,
}

/// The root type representing all languages in languages.yml
/// Maps language names to their Language definitions
pub type Languages = HashMap<String, Language>;

// ============================================================================
// Heuristics types
// ============================================================================

/// Custom deserializer for Option<Vec<String>> that accepts either a single string or an array
fn deserialize_optional_string_or_vec<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrVec {
        Single(String),
        Multiple(Vec<String>),
    }

    Ok(Some(match Option::<StringOrVec>::deserialize(deserializer)? {
        None => return Ok(None),
        Some(StringOrVec::Single(s)) => vec![s],
        Some(StringOrVec::Multiple(v)) => v,
    }))
}

/// Custom deserializer for HashMap<String, Vec<String>> where values can be single strings or arrays
fn deserialize_hashmap_string_or_vec<'de, D>(
    deserializer: D,
) -> Result<HashMap<String, Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrVec {
        Single(String),
        Multiple(Vec<String>),
    }

    let map = HashMap::<String, StringOrVec>::deserialize(deserializer)?;
    Ok(map
        .into_iter()
        .map(|(k, v)| match v {
            StringOrVec::Single(s) => (k, vec![s]),
            StringOrVec::Multiple(v) => (k, v),
        })
        .collect())
}

/// A heuristic rule for disambiguating languages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeuristicRule {
    /// Language(s) to return if this rule matches
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_string_or_vec"
    )]
    pub language: Option<Vec<String>>,

    /// Regex pattern(s) to match (positive)
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_string_or_vec"
    )]
    pub pattern: Option<Vec<String>>,

    /// Regex pattern(s) to match (negative - must NOT match)
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_string_or_vec"
    )]
    pub negative_pattern: Option<Vec<String>>,

    /// Reference to a named pattern
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub named_pattern: Option<String>,

    /// Multiple rules that must all match (AND condition)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<HeuristicRule>>,
}

/// A disambiguation block for a set of file extensions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Disambiguation {
    /// File extensions this block applies to
    pub extensions: Vec<String>,

    /// Ordered list of rules to try
    pub rules: Vec<HeuristicRule>,
}

/// Root structure for heuristics.yml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heuristics {
    /// List of disambiguation blocks
    pub disambiguations: Vec<Disambiguation>,

    /// Map of named patterns (regex patterns) that can be reused by rules
    #[serde(deserialize_with = "deserialize_hashmap_string_or_vec")]
    pub named_patterns: HashMap<String, Vec<String>>,
}

// ============================================================================
// Vendor types
// ============================================================================

/// List of regex patterns for vendored files (vendor.yml)
pub type VendorPatterns = Vec<String>;
