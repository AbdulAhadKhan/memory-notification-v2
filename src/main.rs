use std::{
    collections::HashMap,
    process::{Command, Output},
};

struct ProcessCommand {
    command: Command,
}

impl ProcessCommand {
    fn new() -> Self {
        let mut command = Command::new("ps");
        command.args(&["-e", "-o", "pid,drs", "--sort", "-drs"]);

        Self { command }
    }

    fn execute(&mut self) -> Output {
        self.command.output().expect("failed to execute process")
    }

    fn convert_output_to_map(&mut self) -> HashMap<String, String> {
        let output = self.execute();
        let output = String::from_utf8_lossy(&output.stdout);
        let mut map = HashMap::new();

        for line in output.lines() {
            let mut iter = line.split_whitespace();
            let pid = iter.next().unwrap();
            let drs = iter.next().unwrap();

            map.insert(pid.to_string(), drs.to_string());
        }

        map
    }
}

fn main() {
    let mut process_command = ProcessCommand::new();
    let map = process_command.convert_output_to_map();

    println!("{:?}", map);
}
