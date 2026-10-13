mod test_detection_by_modeline {
    use linguist::detect_language_by_modeline;

    /// Helper: Assert that exactly the expected language is detected
    fn assert_detects_only(content: &str, expected_language: &str) {
        let langs = detect_language_by_modeline(content).expect("Should not error");
        let names: Vec<&str> = langs.iter().map(|lang| lang.name).collect();
        assert_eq!(
            names,
            [expected_language],
            "Expected '{}' for modeline in {:?}",
            expected_language,
            content
        );
    }

    /// Helper: Assert that no language is detected
    fn assert_detects_none(content: &str) {
        let langs = detect_language_by_modeline(content).expect("Should not error");
        assert!(
            langs.is_empty(),
            "Expected no language for {:?}, but got: {:?}",
            content,
            langs.iter().map(|lang| lang.name).collect::<Vec<_>>()
        );
    }

    /// Ported from upstream's `test/fixtures/Data/Modelines` (exercised by
    /// `test_modeline_strategy` in `test/test_strategies.rb`).
    #[test]
    fn upstream_vim_fixtures() {
        let cases = [
            ("/* vim: set filetype=ruby: */\n\n# I am Ruby\n", "Ruby"),
            (
                "/* vim: set ts=8 sw=4 filetype=ruby tw=0: */\n\n# Please help\n",
                "Ruby",
            ),
            (
                "/* vim: set ft=ruby ts=8 sw=4 tw=0: */\n\n# Humor\n",
                "Ruby",
            ),
            ("# vim: filetype=ruby\n\n# I am Ruby\n", "Ruby"),
            ("# vim: ft=ruby\n\n# I am Ruby\n", "Ruby"),
            ("# vim: syntax=Ruby\n\n# I am Ruby\n", "Ruby"),
            ("# vim: se syntax=ruby:\n\n# I am Ruby\n", "Ruby"),
            ("# vim: set syntax=ruby:\n\n# I am Ruby\n", "Ruby"),
            ("# ex: syntax=ruby\n\n# I am Ruby\n", "Ruby"),
            (" ex: noexpandtab: ft=ruby\n\n# Still Ruby\n", "Ruby"),
            ("# vim600: ft=ruby\n\n# Targets Vim 6.0 or later\n", "Ruby"),
            (
                "vim<520: ft=ruby\n\n# Targets Vim 5.20 and earlier\n",
                "Ruby",
            ),
            (
                "/* vim: set ft=cpp: */\n\nI would like to be C++ please.\n",
                "C++",
            ),
            (
                "/* vim: set filetype=pRoloG: */\n\n# I am Prolog\n",
                "Prolog",
            ),
            (
                "# vim: noexpandtab: ft=javascript\n\n\"JS\";\n",
                "JavaScript",
            ),
            // An escaped space keeps `ft=perl` inside the titlestring value
            (
                r#"# vim:noexpandtab titlestring=hi\|there\\\ ft=perl ts=4
# vim:noexpandtab titlestring=hi|there\\ ft=javascript ts=4

"Still JavaScript, bruh";
"#,
                "JavaScript",
            ),
        ];
        for (content, expected) in cases {
            assert_detects_only(content, expected);
        }
    }

    /// Ported from upstream's `test/fixtures/Data/Modelines`.
    #[test]
    fn upstream_emacs_fixtures() {
        let cases = [
            (
                "// -*-c++-*-\ntemplate <typename X> class { X i; };\n",
                "C++",
            ),
            ("// -*- c++ -*-\n", "C++"),
            ("// -*- mode:C++ -*-\n", "C++"),
            ("// -*- font:bar;mode:c++ -*-\n", "C++"),
            ("// -*-foo:bar;mode:c++;bar:foo-*-\n", "C++"),
            ("// -*- foo : bar ; mode : c++ ; bar : foo -*-\n", "C++"),
            ("// -*- mode : c++ ; bar : foo -*-\n", "C++"),
            (
                "// -*- font:x;foo : bar ; mode : C++ ; bar : foo ; foooooo:baaaaar;fo:ba-*-\n",
                "C++",
            ),
            ("// -*-foo:bar;mode:c++;bar:foo;tyrell:corp-*-\n", "C++"),
            ("// -*- foo-bar mode: c++ -*-\n\n\"Malformed\"\n", "C++"),
            ("/* -*- mode: c++ -------*- */\n", "C++"),
            ("-*--------- foo:bar mode: c++ -*-\n", "C++"),
            ("// -*- fundamental -*-\n\nint main() {}\n", "Text"),
            ("; -*-mode:Smalltalk-*-\n", "Smalltalk"),
            ("; -*- MoDe: PhP;-*-\n", "PHP"),
        ];
        for (content, expected) in cases {
            assert_detects_only(content, expected);
        }
    }

    #[test]
    fn vim_help_file() {
        assert_detects_only(
            "Some help text\n vim:tw=78:ts=8:ft=help:norl:\n",
            "Vim Help File",
        );
    }

    #[test]
    fn modeline_in_footer() {
        let content = format!("{}# vim: ft=ruby\n", "filler\n".repeat(20));
        assert_detects_only(&content, "Ruby");
    }

    #[test]
    fn modeline_outside_search_scope_is_ignored() {
        let content = format!(
            "{}# vim: ft=ruby\n{}",
            "filler\n".repeat(10),
            "filler\n".repeat(10)
        );
        assert_detects_none(&content);
    }

    #[test]
    fn strips_bom() {
        assert_detects_only("\u{FEFF}# -*- mode: ruby -*-\n", "Ruby");
    }

    #[test]
    fn vimball_is_ignored() {
        assert_detects_none("\" Vimball Archiver\nUseVimball\nfinish\n\" vim: ft=vim\n");
    }

    #[test]
    fn unknown_mode() {
        assert_detects_none("# vim: ft=notalanguage\n");
    }

    #[test]
    fn no_modeline() {
        assert_detects_none("fn main() {}\n");
        assert_detects_none("");
        // `lex:` is not an `ex:` modeline
        assert_detects_none("flex: ft=ruby\n");
    }
}
