pub mod config_parser;
mod policies;
mod process_command;

use config_parser::Config;
use once_cell::sync::Lazy;
use policies::{process_observer, run_policies, ProcessObserverTrait};
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
        run_policies(&process_observer);

        thread::sleep(time::Duration::from_secs(CONFIGS.core.interval));
    }
}
