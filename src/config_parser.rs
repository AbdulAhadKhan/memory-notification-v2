use std::fs;
use toml::Value;

pub struct ConfigParser {
    config: Value,
}

impl ConfigParser {
    fn retrieve_config() -> Value {
        let config = fs::read_to_string("config.toml").expect("Unable to read config file");
        config
            .parse::<Value>()
            .expect("Unable to parse config file")
    }

    fn convert_value_to_string(value: &Value, default: &str) -> String {
        if value.is_integer() {
            value.as_integer().unwrap().to_string()
        } else if value.is_str() {
            value.as_str().unwrap().to_string()
        } else {
            default.to_string()
        }
    }

    pub fn new() -> ConfigParser {
        let config = ConfigParser::retrieve_config();
        ConfigParser { config }
    }

    pub fn get_value_or_default<'a>(&self, module: &str, key: &str, default: &'a str) -> String {
        self.config
            .get(module)
            .and_then(|server| {
                server
                    .get(key)
                    .and_then(|value| Some(ConfigParser::convert_value_to_string(value, default)))
            })
            .unwrap_or(default.to_string())
            .clone()
    }
}
