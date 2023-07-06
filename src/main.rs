mod config_parser;
mod process_command;

use config_parser::ConfigParser;
use process_command::ProcessCommand;
use std::{thread, time};

fn main() {
    let config = ConfigParser::new();
    let interval = config.get_value_or_default("nonexistent", "name", "1");

    println!("{}", interval);

    loop {
        let mut process_command = ProcessCommand::new();
        let map = process_command.convert_output_to_map();

        println!("{:?}", map);

        thread::sleep(time::Duration::from_secs(1));
    }
}
