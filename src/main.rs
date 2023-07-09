pub mod config_parser;
mod policies;
mod process_command;

use config_parser::ConfigParser;
use policies::{
    p1_log_on_lower, p2_delayed_email_on_upper, p3_lower_upper_lower_spike_log,
    p4_lower_mid_lower_spike_log,
};
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

    let upper_limit = config.get_value_or_default(MODULE, "upper_limit", "10000");
    let upper_limit = upper_limit.parse::<u64>().unwrap();

    let mut process_observer = process_observer::ProcessObserver::new(5);

    loop {
        let mut process_command = ProcessCommand::new(lower_limit);
        let map = process_command.convert_output_to_map();

        process_observer.update_processes(map);
        p1_log_on_lower(&process_observer, lower_limit);
        p2_delayed_email_on_upper(&process_observer, lower_limit);
        p3_lower_upper_lower_spike_log(&process_observer, lower_limit, upper_limit);
        p4_lower_mid_lower_spike_log(&process_observer, lower_limit, upper_limit);

        thread::sleep(time::Duration::from_secs(interval));
    }
}
