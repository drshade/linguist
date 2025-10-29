use linguist::{
    detect_language_by_extension, detect_language_by_filename, disambiguate, is_vendored,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Take filename as argument
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];

    // Is it vendored?
    match is_vendored(filename) {
        Ok(true) => println!("{} -> is vendored", filename),
        Ok(false) => println!("{} -> is not vendored", filename),
        Err(e) => eprintln!("Error checking if vendored: {}", e),
    }

    // By filename
    match detect_language_by_filename(filename) {
        Ok(languages) => {
            for lang in languages {
                println!("{} -> {} (by filename)", filename, lang.name);
            }
        }
        Err(e) => eprintln!("Error detecting by filename: {}", e),
    }

    // By extension
    match detect_language_by_extension(filename) {
        Ok(languages) => {
            for lang in languages {
                println!("{} -> {} (by extension)", filename, lang.name);
            }
        }
        Err(e) => eprintln!("Error detecting by extension: {}", e),
    }

    // Disambiguate
    if let Ok(content) = std::fs::read_to_string(filename) {
        match disambiguate(filename, &content) {
            Ok(languages) => {
                for lang in languages {
                    println!("{} -> {} (by disambiguation)", filename, lang.name);
                }
            }
            Err(e) => eprintln!("Error during disambiguation: {}", e),
        }
    } else {
        eprintln!("Failed to read file: {}", filename);
    }

    Ok(())
}
