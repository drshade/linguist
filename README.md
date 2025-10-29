# Linguist

A Rust library for programming language detection, inspired by [GitHub Linguist](https://github.com/github/linguist). Detects programming languages by file extension, filename, and content-based heuristics.

## Features

- Zero configuration and setup required, just add the crate and call the detection functions 
- Detect languages by exact filename match (e.g., `Makefile`, `Dockerfile`)
- Detect languages by file extension (e.g., `.rs`, `.py`, `.js`)
- Disambiguate between multiple languages using content heuristics
- Identify vendored/third-party files

## Usage

### Detect by Extension

```rust
use linguist::detect_language_by_extension;

let languages = detect_language_by_extension("script.py")?;
assert_eq!(languages[0].0, "Python");
```

### Detect by Filename

```rust
use linguist::detect_language_by_filename;

let languages = detect_language_by_filename("Makefile")?;
assert_eq!(languages[0].0, "Makefile");
```

### Disambiguate by Content

```rust
use linguist::disambiguate;

let content = "#include <iostream>\nint main() {}";
let result = disambiguate("test.h", content)?;
if let Some(languages) = result {
    assert_eq!(languages[0].0, "C++");
}
```

### Check if Vendored

```rust
use linguist::is_vendored;

assert!(is_vendored("node_modules/react/index.js")?);
assert!(!is_vendored("src/main.rs")?);
```

## Acknowledgments

Special thanks to [@vcfxb](https://github.com/vcfxb) for graciously donating the crates.io name "linguist" to this project!

This project is inspired by and uses language definitions from [GitHub Linguist](https://github.com/github/linguist), maintained by GitHub and its contributors. The language definitions (`definitions/languages.yml`, `definitions/heuristics.yml`, `definitions/vendor.yml`) are derived from this project.
