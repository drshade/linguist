mod test_detection_by_extension {
    use linguist::detect_language_by_extension;

    /// Helper: Assert that the detected languages contain the expected language
    fn assert_detects(filename: &str, expected_language: &str) {
        let langs = detect_language_by_extension(filename).expect("Should not error");
        let detected_names: Vec<&str> = langs.iter().map(|lang| lang.name).collect();
        assert!(
            detected_names.contains(&expected_language),
            "Expected '{}' to be detected for '{}', but got: {:?}",
            expected_language,
            filename,
            detected_names
        );
    }

    /// Helper: Assert that only one language is detected and it matches the expected language
    fn assert_detects_only(filename: &str, expected_language: &str) {
        let langs = detect_language_by_extension(filename).expect("Should not error");
        assert_eq!(
            langs.len(),
            1,
            "Expected exactly 1 language for '{}', but got: {}",
            filename,
            langs.len()
        );
        assert_eq!(
            langs[0].name, expected_language,
            "Expected '{}' for '{}', but got '{}'",
            expected_language, filename, langs[0].name
        );
    }

    /// Helper: Assert that no language is detected
    fn assert_detects_none(filename: &str) {
        let langs = detect_language_by_extension(filename).expect("Should not error");
        assert!(
            langs.is_empty(),
            "Expected no language for '{}', but got: {:?}",
            filename,
            langs.iter().map(|lang| lang.name).collect::<Vec<_>>()
        );
    }

    #[test]
    fn detect_rust() {
        // .rs is shared by multiple languages, but Rust should be one of them
        assert_detects("hello.rs", "Rust");
    }

    #[test]
    fn detect_with_path() {
        // .rs is shared by multiple languages, but Rust should be one of them
        assert_detects("src/main.rs", "Rust");
    }

    #[test]
    fn detect_python() {
        assert_detects_only("script.py", "Python");
    }

    #[test]
    fn detect_javascript() {
        assert_detects_only("app.js", "JavaScript");
    }

    #[test]
    fn no_extension() {
        assert_detects_none("Makefile");
    }

    #[test]
    fn unknown_extension() {
        assert_detects_none("file.xyz123");
    }

    #[test]
    fn ambiguous_extension_h() {
        // .h is used by both C and C++
        assert_detects("header.h", "C");
        assert_detects("header.h", "C++");
    }

    #[test]
    fn case_sensitive() {
        // Extensions are case-sensitive
        assert_detects_none("file.RS");
    }

    #[test]
    fn double_extension() {
        assert_detects_only("config.rs.in", "Rust");
    }

    #[test]
    fn common_languages() {
        assert_detects_only("main.go", "Go");
        assert_detects_only("App.java", "Java");
        assert_detects_only("script.rb", "Ruby");
        assert_detects_only("styles.css", "CSS");
    }

    #[test]
    fn compound_extensions() {
        // Test compound extensions like .blade.php, .d.ts, etc.
        assert_detects("template.blade.php", "Blade");
        assert_detects("types.d.ts", "TypeScript");
    }

    #[test]
    fn markdown_variants() {
        // .md is ambiguous between Markdown and GCC Machine Description
        assert_detects("README.md", "Markdown");
    }

    #[test]
    fn txt_extension_is_deterministic() {
        // .txt matches multiple languages (Adblock Filter List, Text, Vim Help File).
        // With BTreeSet, order is always alphabetical. With HashSet, order varies
        // per process due to random hash seeds — run this test in a shell loop
        // across multiple processes to catch non-determinism.
        let names: Vec<&str> = detect_language_by_extension("test.txt")
            .unwrap()
            .iter()
            .map(|l| l.name)
            .collect();
        assert_eq!(
            names,
            vec!["Adblock Filter List", "Text", "Vim Help File"],
            "languages should be in alphabetical order"
        );
    }

    #[test]
    fn returns_language_objects() {
        // Verify we get full Language objects with metadata
        let langs = detect_language_by_extension("script.py").expect("Should not error");
        assert_eq!(langs.len(), 1);

        let detected = &langs[0];
        assert_eq!(detected.name, "Python");
        assert_eq!(detected.definition.ace_mode, "python");
        assert_eq!(detected.definition.tm_scope, "source.python");
        assert!(
            detected
                .definition
                .extensions
                .as_ref()
                .unwrap()
                .contains(&".py".to_string())
        );
        assert!(detected.definition.color.is_some());
    }
}
