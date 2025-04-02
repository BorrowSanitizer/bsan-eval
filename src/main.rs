use std::path::PathBuf;
static TOOL_DIR: &str = "tools";
use serde_json::Result;
mod tool;
mod utils;

fn main() {
    let _args = std::env::args();
    let json_dir = PathBuf::from(TOOL_DIR);

    let json_files = utils::collect_json_files(&json_dir);
    if json_files.is_err() {
        eprintln!("Error collecting JSON files: {:?}", json_files.err());
        return;
    }

    let tools = json_files
        .unwrap()
        .drain(..)
        .map(tool::compile_tools)
        .collect::<Vec<Result<_>>>();

    let (errs, tools): (Vec<_>, Vec<_>) = tools.into_iter().partition(|t| t.is_err());

    if !errs.is_empty() {
        for err in errs {
            eprintln!("Error compiling tool: {:?}", err.err());
        }
    }
    let _ = tools.iter().flatten().flatten().collect::<Vec<_>>();
}
