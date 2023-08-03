pub mod config_parser;
mod email;
mod policies;
mod process_command;

use clap::Parser;
use config_parser::Config;
use home;
use once_cell::sync::Lazy;
use policies::{process_observer, run_policies, ProcessObserverTrait};
use process_command::ProcessCommand;
use std::{thread, time};

#[derive(Parser, Debug)]
#[command(version, about, author)]
struct Opts {
    #[clap(short, long)]
    config: Option<String>,
}

pub static CONFIGS: Lazy<Config> = Lazy::new(|| {
    let opts: Opts = Opts::parse();
    match opts.config {
        Some(config) => Config::new(&config),
        None => {
            let home = home::home_dir().expect("Unable to get home directory");
            let config_path = home.join(".config").join("mnsv2").join("config.toml");
            Config::new(config_path.to_str().unwrap())
        }
    }
});

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
