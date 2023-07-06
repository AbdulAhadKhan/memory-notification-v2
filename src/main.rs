mod config_parser;
mod process_command;

use config_parser::ConfigParser;
use process_command::ProcessCommand;
use std::{thread, time};

const MODULE: &str = "core";

fn main() {
    let config = ConfigParser::new();
    let interval = config.get_value_or_default(MODULE, "interval", "1");
    let interval = interval.parse::<u64>().unwrap();

    loop {
        let mut process_command = ProcessCommand::new();
        let map = process_command.convert_output_to_map();

        println!("{:?}", map);

        thread::sleep(time::Duration::from_secs(interval));
    }
}
