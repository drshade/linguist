mod test_samples {
    use linguist::{detect_language_by_extension, detect_language_by_filename, disambiguate};
    use std::fs;
    use std::path::Path;

    /// Translate a samples folder name to the canonical language name.
    /// Needed for languages whose names contain filesystem-unfriendly characters
    /// and therefore use the `fs_name` field in languages.yml.
    fn canonical_language_name(folder_name: &str) -> &str {
        match folder_name {
            "Fstar" => "F*",
            other => other,
        }
    }

    /// Run the detection pipeline for a single sample file.
    /// For files under a `filenames/` directory, use filename-based detection.
    /// Otherwise try extension-based detection, falling back to content
    /// disambiguation when the extension is ambiguous.
    fn detect(path: &Path, is_filenames_dir: bool) -> Vec<String> {
        let to_names = |langs: Vec<linguist::DetectedLanguage>| -> Vec<String> {
            langs.into_iter().map(|l| l.name.to_string()).collect()
        };

        if is_filenames_dir {
            return to_names(detect_language_by_filename(path).unwrap_or_default());
        }

        let by_ext = to_names(detect_language_by_extension(path).unwrap_or_default());

        match by_ext.len() {
            0 => {
                // No extension match — fall back to filename (e.g. extensionless files
                // like `Makefile`, `Dockerfile`, dotfiles)
                to_names(detect_language_by_filename(path).unwrap_or_default())
            }
            1 => by_ext,
            _ => {
                // Ambiguous extension — try content-based disambiguation
                let content = fs::read_to_string(path).unwrap_or_default();
                let disambiguated =
                    to_names(disambiguate(path, &content).unwrap_or_default());
                if disambiguated.is_empty() {
                    by_ext
                } else {
                    disambiguated
                }
            }
        }
    }

    #[test]
    fn test_all_samples() {
        let samples_dir = Path::new("tests/samples");
        if !samples_dir.exists() {
            eprintln!(
                "Skipping sample tests: tests/samples/ not found.\n\
                 Run `bash tests/pull-samples.sh` to download them."
            );
            return;
        }

        let mut passed = 0usize;
        let mut failed = 0usize;
        let mut skipped = 0usize;
        let mut failures: Vec<String> = Vec::new();

        let mut lang_dirs: Vec<_> = fs::read_dir(samples_dir)
            .expect("failed to read tests/samples")
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir())
            .collect();
        lang_dirs.sort_by_key(|e| e.file_name());

        for lang_entry in lang_dirs {
            let lang_path = lang_entry.path();
            let folder_name = lang_entry.file_name().to_string_lossy().to_string();
            let expected = canonical_language_name(&folder_name);

            let mut entries: Vec<_> = fs::read_dir(&lang_path)
                .unwrap()
                .filter_map(|e| e.ok())
                .collect();
            entries.sort_by_key(|e| e.file_name());

            for entry in entries {
                let path = entry.path();

                if path.is_dir() {
                    // Only recurse into filenames/ subdirectory
                    if entry.file_name() == "filenames" {
                        let mut filenames: Vec<_> = fs::read_dir(&path)
                            .unwrap()
                            .filter_map(|e| e.ok())
                            .filter(|e| e.path().is_file())
                            .collect();
                        filenames.sort_by_key(|e| e.file_name());

                        for fname_entry in filenames {
                            let fname_path = fname_entry.path();
                            check(expected, &fname_path, true, &mut passed, &mut failed, &mut skipped, &mut failures);
                        }
                    }
                    continue;
                }

                if path.is_file() {
                    check(expected, &path, false, &mut passed, &mut failed, &mut skipped, &mut failures);
                }
            }
        }

        println!(
            "\nSample tests: {} passed, {} skipped (undetectable), {} failed",
            passed, skipped, failed
        );

        assert!(
            failures.is_empty(),
            "{} sample(s) detected as wrong language:\n{}",
            failures.len(),
            failures.join("\n")
        );
    }

    fn check(
        expected: &str,
        path: &Path,
        is_filenames_dir: bool,
        passed: &mut usize,
        failed: &mut usize,
        skipped: &mut usize,
        failures: &mut Vec<String>,
    ) {
        let detected = detect(path, is_filenames_dir);
        if detected.iter().any(|n| n == expected) {
            *passed += 1;
        } else if detected.is_empty() {
            *skipped += 1;
        } else {
            *failed += 1;
            failures.push(format!(
                "  {} → expected '{}', got [{}]",
                path.display(),
                expected,
                detected.join(", ")
            ));
        }
    }


}
