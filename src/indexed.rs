use crate::definitions;
use linguist_types::Disambiguation;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{HashMap, HashSet};

//
// Indexed structures for faster lookups etc
//

pub type Filename = String;
pub type Extension = String;
pub type LanguageName = String;

pub static LANGUAGES_BY_FILENAME: Lazy<HashMap<Filename, HashSet<LanguageName>>> =
    Lazy::new(|| {
        // Process the LANGUAGES struct, building up the index
        //
        let mut index = HashMap::new();

        for (lang_name, lang_def) in definitions::LANGUAGES.iter() {
            if let Some(ref filenames) = lang_def.filenames {
                for filename in filenames {
                    index
                        .entry(filename.clone())
                        .or_insert_with(HashSet::new)
                        .insert(lang_name.clone());
                }
            }
        }

        index
    });

pub static LANGUAGES_BY_EXTENSION: Lazy<HashMap<Extension, HashSet<LanguageName>>> =
    Lazy::new(|| {
        // Process the LANGUAGES struct, building up the index
        //
        let mut index = HashMap::new();

        for (lang_name, lang_def) in definitions::LANGUAGES.iter() {
            if let Some(ref extensions) = lang_def.extensions {
                for extension in extensions {
                    index
                        .entry(extension.clone())
                        .or_insert_with(HashSet::new)
                        .insert(lang_name.clone());
                }
            }
        }

        index
    });

pub static DISAMBIGUATIONS_BY_EXTENSION: Lazy<HashMap<Extension, Vec<Disambiguation>>> =
    Lazy::new(|| {
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

pub static VENDOR_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
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
