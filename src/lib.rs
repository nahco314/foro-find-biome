mod finding;

use crate::finding::find_biome;
use onefmt_plugin_utils::data_json_utils::JsonGetter;
pub use onefmt_plugin_utils::interface::{of_free, of_malloc};
use onefmt_plugin_utils::main_from;
use serde_json::{json, Value};
use std::path::PathBuf;

pub fn main_with_json(input: Value) -> Value {
    let current_dir = String::get_value(&input, ["current-dir"]).unwrap();

    let result = match find_biome(&PathBuf::from(current_dir)) {
        Some(v) => {
            json!({
                "found": true,
                "biome": v.to_str().unwrap()
            })
        }
        None => {
            json!({
                "found": false
            })
        }
    };

    result
}

main_from!(main_with_json);
