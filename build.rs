use linguist_types::{Heuristics, Languages, VendorPatterns};
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));

    // Parse and serialize languages.yml
    println!("cargo:rerun-if-changed=definitions/languages.yml");
    let languages: Languages = {
        let file = File::open("definitions/languages.yml").expect("Failed to open languages.yml");
        let reader = BufReader::new(file);
        serde_yaml_ng::from_reader(reader).expect("Failed to parse languages.yml")
    };
    let languages_json = serde_json::to_vec(&languages)
        .expect("Failed to serialize languages to JSON");
    std::fs::write(out_dir.join("languages.json"), &languages_json)
        .expect("Failed to write languages.json");
    println!("cargo:warning=Generated languages.json with {} bytes", languages_json.len());

    // Parse and serialize heuristics.yml
    println!("cargo:rerun-if-changed=definitions/heuristics.yml");
    let heuristics: Heuristics = {
        let file = File::open("definitions/heuristics.yml").expect("Failed to open heuristics.yml");
        let reader = BufReader::new(file);
        serde_yaml_ng::from_reader(reader).expect("Failed to parse heuristics.yml")
    };
    let heuristics_json = serde_json::to_vec(&heuristics)
        .expect("Failed to serialize heuristics to JSON");
    std::fs::write(out_dir.join("heuristics.json"), &heuristics_json)
        .expect("Failed to write heuristics.json");
    println!("cargo:warning=Generated heuristics.json with {} bytes", heuristics_json.len());

    // Parse and serialize vendor.yml
    println!("cargo:rerun-if-changed=definitions/vendor.yml");
    let vendor: VendorPatterns = {
        let file = File::open("definitions/vendor.yml").expect("Failed to open vendor.yml");
        let reader = BufReader::new(file);
        serde_yaml_ng::from_reader(reader).expect("Failed to parse vendor.yml")
    };
    let vendor_json = serde_json::to_vec(&vendor)
        .expect("Failed to serialize vendor to JSON");
    std::fs::write(out_dir.join("vendor.json"), &vendor_json)
        .expect("Failed to write vendor.json");
    println!("cargo:warning=Generated vendor.json with {} bytes", vendor_json.len());
}
