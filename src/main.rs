pub mod config_parser;
mod policies;
mod process_command;

use config_parser::Config;
use once_cell::sync::Lazy;
use policies::{
    p1_log_on_lower, p2_delayed_email_on_upper, p3_lower_upper_lower_spike_log,
    p4_lower_mid_lower_spike_log, p5_lower_upper_mid_spike_log,
};
use policies::{process_observer, ProcessObserverTrait};
use process_command::ProcessCommand;
use std::{thread, time};

pub static CONFIGS: Lazy<Config> = Lazy::new(|| Config::new());

fn main() {
    let mut process_observer =
        process_observer::ProcessObserver::new(CONFIGS.core.observation_window);

    loop {
        let mut process_command = ProcessCommand::new();
        let map = process_command.convert_output_to_map();

        process_observer.update_processes(map);
        p1_log_on_lower(&process_observer);
        p2_delayed_email_on_upper(&process_observer);
        p3_lower_upper_lower_spike_log(&process_observer);
        p4_lower_mid_lower_spike_log(&process_observer);
        p5_lower_upper_mid_spike_log(&process_observer);

        thread::sleep(time::Duration::from_secs(CONFIGS.core.interval));
    }
}
