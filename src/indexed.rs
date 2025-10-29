use crate::definitions;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub type Filename = String;
pub type Extension = String;
pub type LanguageName = String;

pub static LANGUAGES_BY_FILENAME: Lazy<HashMap<Filename, Vec<LanguageName>>> = Lazy::new(|| {
    // Process the LANGUAGES struct, building up the index
    //
    let mut index = HashMap::new();

    for (lang_name, lang_def) in definitions::LANGUAGES.iter() {
        if let Some(ref filenames) = lang_def.filenames {
            for filename in filenames {
                index
                    .entry(filename.clone())
                    .or_insert_with(Vec::new)
                    .push(lang_name.clone());
            }
        }
    }

    index
});

pub static LANGUAGES_BY_EXTENSION: Lazy<HashMap<Extension, Vec<LanguageName>>> = Lazy::new(|| {
    // Process the LANGUAGES struct, building up the index
    //
    let mut index = HashMap::new();

    for (lang_name, lang_def) in definitions::LANGUAGES.iter() {
        if let Some(ref extensions) = lang_def.extensions {
            for extension in extensions {
                index
                    .entry(extension.clone())
                    .or_insert_with(Vec::new)
                    .push(lang_name.clone());
            }
        }
    }

    index
});
