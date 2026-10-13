mod test_samples {
    use linguist::detect_language;
    use std::fs;
    use std::path::Path;

    /// Translate a samples folder name to the canonical language name.
    /// Needed for languages whose names contain filesystem-unfriendly characters
    /// and therefore use the `fs_name` field in languages.yml.
    fn canonical_language_name(folder_name: &str) -> &str {
        match folder_name {
            "Fstar" => "F*",
            "ProC" => "Pro*C",
            other => other,
        }
    }

    /// Run the full detection pipeline for a single sample file.
    fn detect(path: &Path) -> Vec<String> {
        let content = fs::read_to_string(path).ok();
        detect_language(path, content.as_deref())
            .unwrap_or_default()
            .into_iter()
            .map(|l| l.name.to_string())
            .collect()
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
                            check(
                                expected,
                                &fname_path,
                                &mut passed,
                                &mut failed,
                                &mut skipped,
                                &mut failures,
                            );
                        }
                    }
                    continue;
                }

                if path.is_file() {
                    check(
                        expected,
                        &path,
                        &mut passed,
                        &mut failed,
                        &mut skipped,
                        &mut failures,
                    );
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
        passed: &mut usize,
        failed: &mut usize,
        skipped: &mut usize,
        failures: &mut Vec<String>,
    ) {
        let detected = detect(path);
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
