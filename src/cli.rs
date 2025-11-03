//! Command-line interface definitions using clap.

use clap::Parser;

/// Programming language detection tool based on GitHub Linguist
#[derive(Parser, Debug)]
#[command(
    name = "linguist",
    version,
    about = "Detect programming languages in files",
    long_about = "A fast programming language detection tool based on GitHub Linguist.\n\
                  Detects languages by file extension, exact filename, and content analysis
                  as well as whether they are vendored."
)]
pub struct Cli {
    /// Files to analyze
    #[arg(required = true, value_name = "FILE")]
    pub files: Vec<String>,

    /// Detect by file extension only
    #[arg(short = 'e', long = "by-extension")]
    pub by_extension: bool,

    /// Detect by exact filename only
    #[arg(short = 'f', long = "by-filename")]
    pub by_filename: bool,

    /// Detect by content analysis/heuristics only
    #[arg(short = 'c', long = "by-content")]
    pub by_content: bool,

    /// Use all detection methods (extension, filename, and content)
    #[arg(short = 'a', long = "all")]
    pub all: bool,
}

impl Cli {
    /// Determines which detection methods should be used.
    ///
    /// If no specific method is selected, defaults to all methods.
    /// If --all is specified, it overrides individual selections.
    pub fn detection_methods(&self) -> DetectionMethods {
        // If --all is explicitly specified, use all methods
        if self.all {
            return DetectionMethods {
                by_extension: true,
                by_filename: true,
                by_content: true,
            };
        }

        // If any individual method is specified, use only those
        if self.by_extension || self.by_filename || self.by_content {
            return DetectionMethods {
                by_extension: self.by_extension,
                by_filename: self.by_filename,
                by_content: self.by_content,
            };
        }

        // Default: use all methods if none specified
        DetectionMethods {
            by_extension: true,
            by_filename: true,
            by_content: true,
        }
    }
}

/// Represents which detection methods should be used.
#[derive(Debug, Clone, Copy)]
pub struct DetectionMethods {
    pub by_extension: bool,
    pub by_filename: bool,
    pub by_content: bool,
}
