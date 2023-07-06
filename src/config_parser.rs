use std::fs;
use toml::Value;

pub fn retrieve_config() -> Value {
    let config = fs::read_to_string("config.toml").expect("Unable to read config file");
    config
        .parse::<Value>()
        .expect("Unable to parse config file")
}

pub fn get_value_or_default<'a>(
    map: &'a Value,
    module: &str,
    key: &str,
    default: &'a str,
) -> &'a str {
    let test = map
        .get(module)
        .and_then(|server| {
            server.get(key).and_then(|value| {
                if value.is_integer() {
                    Some(value.as_integer().unwrap().to_string())
                } else if value.is_str() {
                    Some(value.as_str().unwrap().to_string())
                } else {
                    None
                }
            })
        })
        .unwrap_or(default.to_string());

    println!("{}", test);
    "hmm"
}
