use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;
use std::ops::Range;

#[derive(Serialize, Deserialize)]
struct RawTool {
    name: String,
    rustflags: Option<Vec<String>>,
    cflags: Option<Vec<String>>,
    cxxflags: Option<Vec<String>>,
    cargo_wrapper: Option<String>,
    wrapper: Option<String>,
    cargo_wrapperflags: Option<Vec<String>>,
    exec_flags: Option<Vec<String>>,
    wrapperflags: Option<Vec<String>>,
    variables: Option<HashMap<String, Vec<String>>>,
}

#[derive(Debug, Clone)]
pub struct Tool {
    name: String,
    rustflags: Vec<String>,
    cflags: Vec<String>,
    cxxflags: Vec<String>,
    wrapper: Option<String>,
    wrapper_flags: Vec<String>,
}

impl Tool {
    fn sub(&self, dict: &HashMap<&str, &str>) -> Self {
        let mut cpy = self.clone();
        cpy.name = sub(cpy.name, dict);
        cpy.rustflags = cpy
            .rustflags
            .iter()
            .map(|f| sub(f.to_string(), dict))
            .collect();
        cpy.cflags = cpy
            .cflags
            .iter()
            .map(|f| sub(f.to_string(), dict))
            .collect();
        cpy.cxxflags = cpy
            .cflags
            .iter()
            .map(|f| sub(f.to_string(), dict))
            .collect();
        cpy.wrapper_flags = cpy
            .wrapper_flags
            .iter()
            .map(|f| sub(f.to_string(), dict))
            .collect();
        cpy.wrapper = cpy.wrapper.map(|s| sub(s, dict));
        cpy
    }
}

lazy_static::lazy_static! {
    static ref SUB_RE: regex::Regex = regex::Regex::new(r"(\@\{(\w+)\})|(\@\^\{(\w+)\})").unwrap();
}

fn sub(mut s: String, dict: &HashMap<&str, &str>) -> String {
    // find all occurrences of @^{varname} or @{varname}
    // find all matches and capture groups
    // iterate over them as tuples of (var:&str, caps: bool)
    // and replace the captured group with the value from the dictionary
    let replacements: Vec<(Range<usize>, &str, bool)> = SUB_RE
        .captures_iter(&s)
        .map(|matc| {
            let (outer, inner, capitalize) = if matc.get(3).is_some() {
                (matc.get(3).unwrap(), matc.get(4).unwrap(), true)
            } else {
                (matc.get(1).unwrap(), matc.get(2).unwrap(), false)
            };
            let value = *dict.get(inner.as_str()).unwrap();
            (outer.range(), value, capitalize)
        })
        .collect();
    for (range, value, capitalize) in replacements {
        if capitalize {
            // capitalize the first letter of the value
            let mut value = value.to_string();
            let first_char = value.chars().next().unwrap();
            let capitalized = first_char.to_uppercase().to_string();
            value.replace_range(0..1, &capitalized);
            s.replace_range(range, value.as_str());
        } else {
            s.replace_range(range, value)
        }
    }
    s
}

pub fn compile_tool(data: String) -> Result<Vec<Tool>> {
    let raw: RawTool = serde_json::from_str(&data)?;
    let base_tool = Tool {
        name: raw.name,
        rustflags: raw.rustflags.unwrap_or_default(),
        cflags: raw.cflags.unwrap_or_default(),
        cxxflags: raw.cxxflags.unwrap_or_default(),
        wrapper: raw.wrapper,
        wrapper_flags: raw.wrapperflags.unwrap_or_default(),
    };
    let result = if let Some(variables) = raw.variables {
        compile_dictionaries(&variables)
            .iter()
            .map(|dict| base_tool.sub(dict))
            .collect::<Vec<Tool>>()
    } else {
        vec![base_tool]
    };
    Ok(result)
}

fn compile_dictionaries(dict: &HashMap<String, Vec<String>>) -> Vec<HashMap<&str, &str>> {
    let mut worklist = Vec::<HashMap<&str, &str>>::new();
    for (key, value_list) in dict.iter() {
        if value_list.is_empty() {
            continue;
        } else {
            let was_empty = worklist.is_empty();
            for elem in value_list {
                if was_empty {
                    let mut map = HashMap::new();
                    map.insert(key.as_str(), elem.as_str());
                    worklist.push(map);
                } else {
                    for mut dict in worklist.clone() {
                        dict.insert(key.as_str(), elem.as_str());
                        worklist.push(dict);
                    }
                }
            }
        }
    }
    worklist
}
