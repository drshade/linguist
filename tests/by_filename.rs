mod test_detection_by_filename {
    use linguist::detect_language_by_filename;

    /// Helper: Assert that only one language is detected and it matches the expected language
    fn assert_detects_only(filename: &str, expected_language: &str) {
        let langs = detect_language_by_filename(filename);
        assert_eq!(
            langs.len(),
            1,
            "Expected exactly 1 language for '{}', but got: {}",
            filename,
            langs.len()
        );
        let (detected_name, _detected_lang) = langs[0];
        assert_eq!(
            detected_name, expected_language,
            "Expected '{}' for '{}', but got '{}'",
            expected_language, filename, detected_name
        );
    }

    /// Helper: Assert that no language is detected
    fn assert_detects_none(filename: &str) {
        let langs = detect_language_by_filename(filename);
        assert!(
            langs.is_empty(),
            "Expected no language for '{}', but got: {:?}",
            filename,
            langs.iter().map(|(name, _)| name).collect::<Vec<_>>()
        );
    }

    #[test]
    fn detect_makefile() {
        assert_detects_only("Makefile", "Makefile");
    }

    #[test]
    fn detect_dockerfile() {
        assert_detects_only("Dockerfile", "Dockerfile");
    }

    #[test]
    fn detect_gitignore() {
        assert_detects_only(".gitignore", "Ignore List");
    }

    #[test]
    fn detect_with_path() {
        // Should work with paths, extracting just the filename
        assert_detects_only("path/to/Makefile", "Makefile");
        assert_detects_only("/etc/Dockerfile", "Dockerfile");
    }

    #[test]
    fn no_match_for_extension() {
        // Should not match extensions - that's what detect_language_by_extension is for
        assert_detects_none("test.py");
        assert_detects_none("script.js");
    }

    #[test]
    fn case_sensitive() {
        // Filenames are case-sensitive exact matches
        // Both "Makefile" and "makefile" are valid (both in the Makefile language definition)
        assert_detects_only("Makefile", "Makefile");
        assert_detects_only("makefile", "Makefile");

        // But completely different case won't match
        assert_detects_none("DOCKERFILE"); // Dockerfile exists, but not DOCKERFILE
    }

    #[test]
    fn detect_apkbuild() {
        assert_detects_only("APKBUILD", "Alpine Abuild");
    }

    #[test]
    fn detect_build_xml() {
        assert_detects_only("build.xml", "Ant Build System");
    }

    #[test]
    fn unknown_filename() {
        assert_detects_none("unknown_special_file");
        assert_detects_none("random.txt");
    }
}
