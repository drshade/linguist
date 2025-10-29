mod test_disambiguate {
    use linguist::disambiguate;

    /// Helper: Assert that disambiguation returns the expected language
    fn assert_disambiguates(filename: &str, content: &str, expected_language: &str) {
        let result = disambiguate(filename, content).expect("Should not error");
        assert!(
            !result.is_empty(),
            "Expected disambiguation to return a result for '{}', but got empty",
            filename
        );

        // Ensure the expected_language is an element in the list of results
        //
        let detected_names: Vec<&str> = result.iter().map(|lang| lang.name).collect();
        assert!(
            detected_names.contains(&expected_language),
            "Expected '{}' for '{}', but got '{:?}'",
            expected_language,
            filename,
            detected_names
        );
    }

    /// Helper: Assert that disambiguation returns empty vec
    fn assert_no_disambiguation(filename: &str, content: &str) {
        let result = disambiguate(filename, content).expect("Should not error");

        assert!(
            result.is_empty(),
            "Expected no disambiguation for '{}', but got: {:?}",
            filename,
            result.iter().map(|lang| lang.name).collect::<Vec<_>>()
        );
    }

    #[test]
    fn detect_c_header() {
        // .h files are ambiguous between C and C++
        // Simple C-style code should be detected as C
        let c_content = r#"
#include <stdio.h>

int main() {
    printf("Hello World\n");
    return 0;
}
"#;
        assert_disambiguates("test.h", c_content, "C");
    }

    #[test]
    fn detect_empty_h_file_as_c() {
        // Empty .h file should fall through Objective-C and C++ rules
        // and match the final C fallback rule (which has no pattern)
        assert_disambiguates("empty.h", "", "C");
    }

    #[test]
    fn detect_cpp_header() {
        // C++ specific features like iostream and std::cout
        let cpp_content = r#"
#include <iostream>

int main() {
    std::cout << "Hello World" << std::endl;
    return 0;
}
"#;
        assert_disambiguates("test.h", cpp_content, "C++");
    }

    #[test]
    fn detect_prolog_vs_perl() {
        // .pl is ambiguous between Prolog and Perl
        // Prolog code with typical patterns
        let prolog_content = r#"
parent(tom, bob).
parent(tom, liz).
parent(bob, ann).

grandparent(X, Z) :-
    parent(X, Y),
    parent(Y, Z).
"#;
        assert_disambiguates("test.pl", prolog_content, "Prolog");
    }

    #[test]
    fn detect_perl_vs_prolog() {
        // Perl code with shebang
        let perl_content = r#"#!/usr/bin/env perl
use strict;
use warnings;

print "Hello World\n";
"#;
        assert_disambiguates("test.pl", perl_content, "Perl");
    }

    #[test]
    fn detect_module_definition() {
        // .mod files can be various things
        // Linux kernel module
        let mod_content = r#"kernel/fs/ext4/ext4.ko
kernel/drivers/net/ethernet/intel/e1000/e1000.ko
"#;
        assert_disambiguates("modules.dep.mod", mod_content, "Linux Kernel Module");
    }

    #[test]
    fn detect_typescript_definition() {
        // .ts files are ambiguous between TypeScript and XML
        // TypeScript with typical patterns
        let ts_content = r#"
interface User {
    name: string;
    age: number;
}

function greet(user: User): void {
    console.log(`Hello ${user.name}`);
}
"#;
        assert_disambiguates("index.ts", ts_content, "TypeScript");
    }

    #[test]
    fn detect_xml_as_ts() {
        // .ts extension with XML content
        let xml_content = r#"<?xml version="1.0"?>
<TS version="2.1" language="en_US">
    <context>
        <name>MainWindow</name>
        <message>
            <source>Hello</source>
            <translation>Hello</translation>
        </message>
    </context>
</TS>
"#;
        assert_disambiguates("translation.ts", xml_content, "XML");
    }

    #[test]
    fn empty_content() {
        // Empty content should still try to disambiguate
        // Depending on the rules, it might match a fallback
        let result = disambiguate("test.h", "").expect("Should not error");
        // We don't assert a specific result here, as it depends on heuristic rules
        // Just verify it doesn't panic
        let _ = result;
    }

    #[test]
    fn no_extension() {
        // File without extension should return None
        assert_no_disambiguation("Makefile", "some content");
    }

    #[test]
    fn unknown_extension() {
        // Extension that has no disambiguation rules
        assert_no_disambiguation("file.xyz123", "some content");
    }

    #[test]
    fn with_path() {
        // Should work with full paths
        let cpp_content = r#"
#include <iostream>
using namespace std;
"#;
        assert_disambiguates("src/include/test.h", cpp_content, "C++");
    }

    #[test]
    fn detect_renderscript() {
        // .rs is ambiguous between Rust and RenderScript
        // RenderScript has specific pragmas
        let rs_content = r#"
#pragma version(1)
#pragma rs java_package_name(com.example.app)

void root(const uchar4 *v_in, uchar4 *v_out) {
    *v_out = *v_in;
}
"#;
        assert_disambiguates("test.rs", rs_content, "RenderScript");
    }

    #[test]
    fn detect_rust_vs_renderscript() {
        // Rust code should be detected correctly
        let rust_content = r#"fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
"#;
        assert_disambiguates("main.rs", rust_content, "Rust");
    }
}
