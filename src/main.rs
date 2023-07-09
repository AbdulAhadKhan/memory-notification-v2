pub mod config_parser;
mod policies;
mod process_command;

use config_parser::ConfigParser;
use policies::{process_observer, ProcessObserverTrait};
use process_command::ProcessCommand;
use std::{thread, time};

const MODULE: &str = "core";

fn main() {
    let config = ConfigParser::new();

    let interval = config.get_value_or_default(MODULE, "interval", "1");
    let interval = interval.parse::<u64>().unwrap();

    let lower_limit = config.get_value_or_default(MODULE, "lower_limit", "5000");
    let lower_limit = lower_limit.parse::<u64>().unwrap();

    let mut process_observer = process_observer::ProcessObserver::new(5);

    loop {
        let mut process_command = ProcessCommand::new(lower_limit);
        let map = process_command.convert_output_to_map();

        println!("Map: {:?}", map);

        process_observer.update_processes(map);
        println!("Observer: {:?}\n", process_observer);

        thread::sleep(time::Duration::from_secs(interval));
    }
}
