use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

#[derive(Serialize, Deserialize)]
struct RawTool {
    name: String,
    cargo_wrapper: Option<String>,
    wrapper: Option<String>,
    modes: Option<HashMap<String, RawFlags>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RawFlags {
    cargo_wrapper_flags: Option<Vec<String>>,
    exec_flags: Option<Vec<String>>,
    wrapper_flags: Option<Vec<String>>,
    rust_flags: Option<Vec<String>>,
    c_flags: Option<Vec<String>>,
    cxx_flags: Option<Vec<String>>,
}

#[derive(Default, Clone, Serialize)]
pub struct Tool {
    name: String,
    cargo_wrapper: Option<String>,
    wrapper: Option<String>,
    flags: Flags,
    modes: Vec<String>,
}

#[derive(Default, Clone, Serialize)]
struct Flags {
    cargo_wrapper_flags: Vec<String>,
    exec_flags: Vec<String>,
    wrapper_flags: Vec<String>,
    rust_flags: Vec<String>,
    c_flags: Vec<String>,
    cxx_flags: Vec<String>,
}

impl Flags {
    fn join(&mut self, raw: &RawFlags) {
        if let Some(flags) = &raw.cargo_wrapper_flags {
            self.cargo_wrapper_flags.extend(flags.clone());
        }
        if let Some(flags) = &raw.exec_flags {
            self.exec_flags.extend(flags.clone());
        }
        if let Some(flags) = &raw.wrapper_flags {
            self.wrapper_flags.extend(flags.clone());
        }
        if let Some(flags) = &raw.rust_flags {
            self.rust_flags.extend(flags.clone());
        }
        if let Some(flags) = &raw.c_flags {
            self.c_flags.extend(flags.clone());
        }
        if let Some(flags) = &raw.cxx_flags {
            self.cxx_flags.extend(flags.clone());
        }
    }
}

impl From<&RawTool> for Tool {
    fn from(raw: &RawTool) -> Self {
        Tool {
            name: raw.name.clone(),
            cargo_wrapper: raw.cargo_wrapper.clone(),
            wrapper: raw.wrapper.clone(),
            flags: Flags::default(),
            modes: Vec::new(),
        }
    }
}

impl Tool {
    fn with_mode(&self, name: String, flags: &RawFlags) -> Self {
        let mut other = self.clone();
        other.modes.push(name);
        other.flags.join(flags);
        other
    }
}

pub fn compile_tools(value: Value) -> Result<Vec<Tool>> {
    let raw: RawTool = serde_json::from_value(value)?;
    let mut base_tool = Tool::from(&raw);
    if let Some(mut modes) = raw.modes {
        if let Some(default_mode) = modes.remove("default") {
            base_tool.flags.join(&default_mode);
        }
        Ok(modes
            .iter()
            .map(|(k, v)| base_tool.with_mode(k.to_string(), v))
            .collect::<Vec<Tool>>())
    } else {
        Ok(vec![base_tool])
    }
}
