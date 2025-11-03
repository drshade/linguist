mod cli;

use clap::Parser;
use cli::{Cli, DetectionMethods};
use linguist::{
    detect_language_by_extension, detect_language_by_filename, disambiguate, is_vendored,
};
use std::process;

fn main() {
    let cli = Cli::parse();
    let methods = cli.detection_methods();

    let mut any_success = false;
    let mut any_error = false;

    for filepath in &cli.files {
        match process_file(filepath, methods) {
            Ok(()) => any_success = true,
            Err(e) => {
                eprintln!("Error processing {}: {}", filepath, e);
                any_error = true;
            }
        }
    }

    // Exit with error code if all files failed
    if !any_success && any_error {
        process::exit(1);
    }
}

fn process_file(
    filepath: &str,
    methods: DetectionMethods,
) -> Result<(), Box<dyn std::error::Error>> {
    // Always check if vendored
    let vendored = is_vendored(filepath).unwrap_or(false);
    let vendored_status = if vendored { "[vendored]" } else { "" };

    let mut found_any = false;

    // Detect by extension
    if methods.by_extension {
        match detect_language_by_extension(filepath) {
            Ok(languages) if !languages.is_empty() => {
                found_any = true;
                let names: Vec<&str> = languages.iter().map(|l| l.name).collect();
                println!(
                    "{}: {} (by extension) {}",
                    filepath,
                    names.join(", "),
                    vendored_status
                );
            }
            Ok(_) => {} // No matches, that's ok
            Err(e) => eprintln!(
                "Warning: Error detecting by extension for {}: {}",
                filepath, e
            ),
        }
    }

    // Detect by filename
    if methods.by_filename {
        match detect_language_by_filename(filepath) {
            Ok(languages) if !languages.is_empty() => {
                found_any = true;
                let names: Vec<&str> = languages.iter().map(|l| l.name).collect();
                println!(
                    "{}: {} (by filename) [{}]",
                    filepath,
                    names.join(", "),
                    vendored_status
                );
            }
            Ok(_) => {} // No matches, that's ok
            Err(e) => eprintln!(
                "Warning: Error detecting by filename for {}: {}",
                filepath, e
            ),
        }
    }

    // Detect by content
    if methods.by_content {
        match std::fs::read_to_string(filepath) {
            Ok(content) => {
                match disambiguate(filepath, &content) {
                    Ok(languages) if !languages.is_empty() => {
                        found_any = true;
                        let names: Vec<&str> = languages.iter().map(|l| l.name).collect();
                        println!(
                            "{}: {} (by content) [{}]",
                            filepath,
                            names.join(", "),
                            vendored_status
                        );
                    }
                    Ok(_) => {} // No matches, that's ok
                    Err(e) => eprintln!(
                        "Warning: Error during disambiguation for {}: {}",
                        filepath, e
                    ),
                }
            }
            Err(e) => {
                eprintln!("Warning: Failed to read file {}: {}", filepath, e);
            }
        }
    }

    // If no language was detected by any method, report as unknown
    if !found_any {
        println!("{}: Unknown [{}]", filepath, vendored_status);
    }

    Ok(())
}
