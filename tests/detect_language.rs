mod test_detect_language {
    use linguist::detect_language;

    fn detected_names(filename: &str, content: Option<&str>) -> Vec<&'static str> {
        detect_language(filename, content)
            .expect("Should not error")
            .iter()
            .map(|lang| lang.name)
            .collect()
    }

    /// Helper: Assert that exactly the expected language is detected
    fn assert_detects_only(filename: &str, content: Option<&str>, expected_language: &str) {
        assert_eq!(
            detected_names(filename, content),
            [expected_language],
            "Expected only '{}' for '{}'",
            expected_language,
            filename
        );
    }

    #[test]
    fn unambiguous_extension() {
        assert_detects_only("script.py", None, "Python");
        assert_detects_only("src/main.rs", Some("fn main() {}\n"), "Rust");
    }

    #[test]
    fn exact_filename() {
        assert_detects_only("Makefile", None, "Makefile");
        assert_detects_only("project/.gitignore", None, "Ignore List");
    }

    #[test]
    fn filename_beats_ambiguous_extension() {
        // `.txt` is ambiguous, but the exact filename settles it
        assert_detects_only("CMakeLists.txt", Some("project(foo)\n"), "CMake");
    }

    #[test]
    fn ambiguous_extension_resolved_by_content() {
        assert_detects_only(
            "widget.h",
            Some("#include <iostream>\nclass Widget {};\n"),
            "C++",
        );
    }

    #[test]
    fn ambiguous_extension_without_content_returns_candidates() {
        let names = detected_names("widget.h", None);
        assert!(
            names.len() > 1,
            "Expected several candidates, got {names:?}"
        );
        assert!(names.contains(&"C"));
        assert!(names.contains(&"C++"));
    }

    #[test]
    fn ambiguous_filename_without_content_returns_candidates() {
        assert_eq!(detected_names(".releaserc", None), ["JSON", "YAML"]);
    }

    #[test]
    fn modeline_overrides_extension() {
        assert_detects_only("notes.txt", Some("# vim: ft=ruby\n"), "Ruby");
        assert_detects_only("Makefile", Some("# -*- mode: python -*-\n"), "Python");
    }

    #[test]
    fn modeline_resolves_vim_help() {
        assert_detects_only(
            "doc/plugin.txt",
            Some("*plugin.txt*  A plugin\n\n vim:tw=78:ts=8:ft=help:norl:\n"),
            "Vim Help File",
        );
    }

    #[test]
    fn unknown_file() {
        assert!(detected_names("mystery.zzzunknown", Some("???\n")).is_empty());
        assert!(detected_names("mystery", None).is_empty());
    }
}
