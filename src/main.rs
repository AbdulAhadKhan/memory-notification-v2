mod process_command;
use process_command::ProcessCommand;
use std::{thread, time};

fn main() {
    loop {
        let mut process_command = ProcessCommand::new();
        let map = process_command.convert_output_to_map();

        println!("{:?}", map);

        thread::sleep(time::Duration::from_secs(1));
    }
}
