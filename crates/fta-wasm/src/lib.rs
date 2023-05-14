use fta::{analyze_file, parse_module::parse_module};
use serde_json::{json, to_string, Value};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[cfg(test)]
mod lib_tests;

#[wasm_bindgen]
pub fn analyze_file_wasm(source_code: &str) -> String {
    let json_string;

    match parse_module(source_code) {
        (Ok(module), line_count) => {
            let (cyclo, halstead_metrics, fta_score) = analyze_file(&module, line_count);
            let mut analyzed: HashMap<&str, Value> = HashMap::new();
            analyzed.insert("line_count", json!(line_count));
            analyzed.insert("cyclo", json!(cyclo));
            analyzed.insert("halstead_metrics", json!(halstead_metrics));
            analyzed.insert("fta_score", json!(fta_score));
            json_string = to_string(&analyzed).unwrap();
        }
        (Err(_err), _) => {
            wasm_bindgen::throw_str("Unable to parse module");
        }
    }

    json_string
}
