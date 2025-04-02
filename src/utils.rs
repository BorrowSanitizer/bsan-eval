use std::path::PathBuf;

use serde_json::Result;
use serde_json::Value;
use std::fs;

pub fn collect_json_files(path: &PathBuf) -> Result<Vec<Value>> {
    fs::read_dir(path)
        .unwrap()
        .filter_map(|entry| {
            entry.map_or(None, |e| {
                let path = e.path();
                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    let data = fs::read_to_string(path).unwrap();
                    Some(serde_json::from_str(&data))
                } else {
                    None
                }
            })
        })
        .collect()
}
