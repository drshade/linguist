# Linguist

A Rust library for programming language detection, inspired by [GitHub Linguist](https://github.com/github/linguist). Detects programming languages by file extension, filename, editor modelines, and content-based heuristics.

## Features

- Zero configuration and setup required, just add the crate and call `detect_language`
- One call runs every strategy in upstream linguist's order, or use each on its own
- Detect languages by exact filename match (e.g., `Makefile`, `Dockerfile`)
- Detect languages by file extension (e.g., `.rs`, `.py`, `.js`)
- Detect languages from Vim and Emacs modelines (e.g., `vim: ft=ruby`, `-*- mode: ruby -*-`)
- Disambiguate between multiple languages using content heuristics
- Identify vendored/third-party files

## Usage

### Detect a File's Language

`detect_language` runs modeline, filename, extension and heuristic detection in turn,
stopping as soon as one language remains. It returns one language when detection is
conclusive, several candidates when the file is still ambiguous, and none when it's unknown.

```rust
use linguist::detect_language;

let content = std::fs::read_to_string("src/widget.h").ok();
let languages = detect_language("src/widget.h", content.as_deref())?;
if let [language] = languages.as_slice() {
    println!("{}", language.name);
}
```

Pass `None` for the content when it isn't available (for example, binary files);
only the filename and extension are considered then.

The individual strategies below are available for callers that need just one.

### Detect by Extension

```rust
use linguist::detect_language_by_extension;

let languages = detect_language_by_extension("script.py")?;
assert_eq!(languages[0].name, "Python");
```

### Detect by Filename

```rust
use linguist::detect_language_by_filename;

let languages = detect_language_by_filename("Makefile")?;
assert_eq!(languages[0].name, "Makefile");
```

### Disambiguate by Content

```rust
use linguist::disambiguate;

let content = "#include <iostream>\nint main() {}";
let languages = disambiguate("test.h", content)?;
assert_eq!(languages[0].name, "C++");
```

### Detect by Modeline

```rust
use linguist::detect_language_by_modeline;

let languages = detect_language_by_modeline("# -*- mode: ruby -*-\nputs 'Hello'\n")?;
assert_eq!(languages[0].name, "Ruby");
```

### Check if Vendored

```rust
use linguist::is_vendored;

assert!(is_vendored("node_modules/react/index.js")?);
assert!(!is_vendored("src/main.rs")?);
```

## Command-line tool

A small CLI is included behind the optional `cli` feature (so library users don't pull in its dependencies):

```bash
cargo install linguist --features cli
linguist src/main.rs Makefile script.py
```

## Acknowledgments

Special thanks to [@vcfxb](https://github.com/vcfxb) for graciously donating the crates.io name "linguist" to this project!

This project is inspired by and uses language definitions from [GitHub Linguist](https://github.com/github/linguist), maintained by GitHub and its contributors. The language definitions (`definitions/languages.yml`, `definitions/heuristics.yml`, `definitions/vendor.yml`) are derived from this project.

## Contributing

Contributions welcome — see [CONTRIBUTING.md](CONTRIBUTING.md).

## Maintaining

See [MAINTAINING.md](MAINTAINING.md) for how to sync definitions and test samples with upstream github-linguist.
