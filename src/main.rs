mod process_command;
use process_command::ProcessCommand;
use std::{fs, thread, time};
use toml::Value;

fn retrieve_config() -> Value {
    let config = fs::read_to_string("config.toml").expect("Unable to read config file");
    config
        .parse::<Value>()
        .expect("Unable to parse config file")
}

fn get_value_or_default<'a>(map: &'a Value, module: &str, key: &str, default: &'a str) -> &'a str {
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

fn main() {
    let config = retrieve_config();
    let interval = get_value_or_default(&config, "core", "interval", "1");

    println!("{}", interval);

    loop {
        let mut process_command = ProcessCommand::new();
        let map = process_command.convert_output_to_map();

        println!("{:?}", map);

        thread::sleep(time::Duration::from_secs(1));
    }
}
