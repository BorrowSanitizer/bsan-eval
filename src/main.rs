use std::{fs, path::PathBuf};
static TOOL_DIR: &str = "tools";

mod tool;
fn main() {
    let _args = std::env::args();
    let json_dir = PathBuf::from(TOOL_DIR);
    collect_json_files(&json_dir).iter().for_each(|json_file| {
        let data = fs::read_to_string(json_file).unwrap();
        match tool::compile_tool(data) {
            Ok(tools) => {
                for tool in tools {
                    println!("{:?}", tool);
                }
            }
            Err(e) => eprintln!("Error parsing JSON: {}", e),
        }
    });
}

fn collect_json_files(path: &PathBuf) -> Vec<PathBuf> {
    fs::read_dir(path)
        .unwrap()
        .filter_map(|entry| {
            entry.map_or(None, |e| {
                let path = e.path();
                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    Some(path)
                } else {
                    None
                }
            })
        })
        .collect()
}
