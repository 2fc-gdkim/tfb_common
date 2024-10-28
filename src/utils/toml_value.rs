
pub fn get_string_from_map(node : &toml::map::Map<String, toml::Value>, key : &str, default_value: &str) -> String {
    match node.get(key) {
        Some(v) => v.as_str().unwrap_or(default_value).to_string(),
        None => return default_value.to_string(),
    }
}

pub fn get_i64_from_map(node : &toml::map::Map<String, toml::Value>, key : &str, default_value: i64) -> i64 {
    match node.get(key) {
        Some(v) => v.as_integer().unwrap_or(default_value) as i64,
        None => return default_value,
    }
}

pub fn get_string_from_value(val : &toml::Value, key : &str, default_value: &str) -> String {
    match val.get(key) {
       Some(v) => return v.as_str().unwrap_or(default_value).to_string(),
       None => return default_value.to_string(),
    };
}

pub fn get_i64_from_value(val : &toml::Value, key : &str, default_value: i64) -> i64 {
    match val.get(key) {
       Some(v) => return v.as_integer().unwrap_or(default_value) as i64,
       None => return default_value,
    };
}

pub fn get_bool_from_value(val : &toml::Value, key : &str, default_value: bool) -> bool {
    match val.get(key) {
       Some(v) => return v.as_bool().unwrap_or(default_value),
       None => return default_value,
    };
}
