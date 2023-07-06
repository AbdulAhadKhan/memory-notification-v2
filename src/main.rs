mod config_parser;
mod process_command;

use config_parser::{get_value_or_default, retrieve_config};
use process_command::ProcessCommand;
use std::{thread, time};

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
