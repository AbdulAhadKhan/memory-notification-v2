mod process_command;
use process_command::ProcessCommand;

fn main() {
    let mut process_command = ProcessCommand::new();
    let map = process_command.convert_output_to_map();

    println!("{:?}", map);
}
