mod test_vendored {
    use linguist::is_vendored;

    #[test]
    fn detect_node_modules() {
        assert!(is_vendored("node_modules/react/index.js"));
        assert!(is_vendored("frontend/node_modules/lodash/main.js"));
        assert!(is_vendored("node_modules/@types/node/index.d.ts"));
    }

    #[test]
    fn detect_bower_components() {
        assert!(is_vendored("bower_components/jquery/dist/jquery.js"));
        assert!(is_vendored("public/bower_components/angular/angular.js"));
    }

    #[test]
    fn detect_vendor_directories() {
        assert!(is_vendored("vendor/bundle/ruby/2.7.0/gems/rails.rb"));
        assert!(is_vendored("vendors/third-party/lib.js"));
        assert!(is_vendored("app/vendor/package/main.go"));
    }

    #[test]
    fn detect_dependencies() {
        assert!(is_vendored("Dependencies/framework/lib.swift"));
        assert!(is_vendored("dependencies/external/module.py"));
    }

    #[test]
    fn detect_dist_directories() {
        assert!(is_vendored("dist/bundle.js"));
        assert!(is_vendored("build/dist/output.min.js"));
    }

    #[test]
    fn detect_cache_directories() {
        assert!(is_vendored("cache/compiled.js"));
        assert!(is_vendored("build/cache/data.json"));
    }

    #[test]
    fn detect_autoconf_files() {
        assert!(is_vendored("configure"));
        assert!(is_vendored("build/configure"));
        assert!(is_vendored("config.guess"));
        assert!(is_vendored("config.sub"));
        assert!(is_vendored("aclocal.m4"));
    }

    #[test]
    fn not_vendored_normal_files() {
        assert!(!is_vendored("src/main.rs"));
        assert!(!is_vendored("lib/utils.js"));
        assert!(!is_vendored("app/controllers/user_controller.rb"));
        assert!(!is_vendored("test/test_helper.py"));
        assert!(!is_vendored("README.md"));
    }

    #[test]
    fn not_vendored_similar_names() {
        // Files that contain "vendor" or "node" but aren't in vendor directories
        assert!(!is_vendored("src/vendor_api.rs"));
        assert!(!is_vendored("lib/node_parser.js"));
        assert!(!is_vendored("app/vendor_controller.rb"));
    }

    #[test]
    fn absolute_paths() {
        assert!(is_vendored("/home/user/project/node_modules/pkg/index.js"));
        assert!(is_vendored("/var/www/app/vendor/lib/file.rb"));
    }

    #[test]
    fn empty_path() {
        assert!(!is_vendored(""));
    }

    #[test]
    fn just_filename() {
        // configure is matched as a vendored file
        assert!(is_vendored("configure"));

        // But regular filenames are not
        assert!(!is_vendored("main.js"));
        assert!(!is_vendored("index.html"));
    }
}
