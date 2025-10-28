mod test_vendored {
    use linguist::is_vendored;

    /// Helper: Assert that a file is vendored (expects no error)
    fn assert_vendored(path: &str) {
        assert!(
            is_vendored(path).expect("Should not error"),
            "Expected '{}' to be vendored, but it was not",
            path
        );
    }

    /// Helper: Assert that a file is not vendored (expects no error)
    fn assert_not_vendored(path: &str) {
        assert!(
            !is_vendored(path).expect("Should not error"),
            "Expected '{}' to not be vendored, but it was",
            path
        );
    }

    #[test]
    fn detect_node_modules() {
        assert_vendored("node_modules/react/index.js");
        assert_vendored("frontend/node_modules/lodash/main.js");
        assert_vendored("node_modules/@types/node/index.d.ts");
    }

    #[test]
    fn detect_bower_components() {
        assert_vendored("bower_components/jquery/dist/jquery.js");
        assert_vendored("public/bower_components/angular/angular.js");
    }

    #[test]
    fn detect_vendor_directories() {
        assert_vendored("vendor/bundle/ruby/2.7.0/gems/rails.rb");
        assert_vendored("vendors/third-party/lib.js");
        assert_vendored("app/vendor/package/main.go");
    }

    #[test]
    fn detect_dependencies() {
        assert_vendored("Dependencies/framework/lib.swift");
        assert_vendored("dependencies/external/module.py");
    }

    #[test]
    fn detect_dist_directories() {
        assert_vendored("dist/bundle.js");
        assert_vendored("build/dist/output.min.js");
    }

    #[test]
    fn detect_cache_directories() {
        assert_vendored("cache/compiled.js");
        assert_vendored("build/cache/data.json");
    }

    #[test]
    fn detect_autoconf_files() {
        assert_vendored("configure");
        assert_vendored("build/configure");
        assert_vendored("config.guess");
        assert_vendored("config.sub");
        assert_vendored("aclocal.m4");
    }

    #[test]
    fn not_vendored_normal_files() {
        assert_not_vendored("src/main.rs");
        assert_not_vendored("lib/utils.js");
        assert_not_vendored("app/controllers/user_controller.rb");
        assert_not_vendored("test/test_helper.py");
        assert_not_vendored("README.md");
    }

    #[test]
    fn not_vendored_similar_names() {
        // Files that contain "vendor" or "node" but aren't in vendor directories
        assert_not_vendored("src/vendor_api.rs");
        assert_not_vendored("lib/node_parser.js");
        assert_not_vendored("app/vendor_controller.rb");
    }

    #[test]
    fn absolute_paths() {
        assert_vendored("/home/user/project/node_modules/pkg/index.js");
        assert_vendored("/var/www/app/vendor/lib/file.rb");
    }

    #[test]
    fn empty_path() {
        // Empty path is valid (current directory) and should not be vendored
        assert_not_vendored("");
    }

    #[test]
    fn just_filename() {
        // configure is matched as a vendored file
        assert_vendored("configure");

        // But regular filenames are not
        assert_not_vendored("main.js");
        assert_not_vendored("index.html");
    }
}
