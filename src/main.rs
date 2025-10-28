use linguist::definitions::{HEURISTICS, LANGUAGES, VENDOR};
use linguist::{
    detect_language_by_extension, detect_language_by_filename, disambiguate, is_vendored,
};

fn main() {
    // Take filename as argument
    //
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];

    // Is it vendored?
    //
    if is_vendored(filename.clone()) {
        println!("{} -> is vendored", filename);
    } else {
        println!("{} -> is not vendored", filename);
    }

    // By filename
    //
    for (name, lang) in detect_language_by_filename(filename.clone()) {
        println!("{} -> {} (by filename)", filename, name);
    }

    // By extension
    //
    for (name, lang) in detect_language_by_extension(filename.clone()) {
        println!("{} -> {} (by extension)", filename, name);
    }

    // Disambiguate
    //
    if let Ok(content) = std::fs::read_to_string(filename) {
        if let Some(languages) = disambiguate(filename, &content) {
            for (name, lang) in languages {
                println!("{} -> {} (by disambiguation)", filename, name);
            }
        } else {
            println!("{} could not be disambiguated", filename);
        }
    } else {
        eprintln!("Failed to read file: {}", filename);
    }
}
