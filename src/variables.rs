
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
