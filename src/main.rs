use std::process::{Command, Output};

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
}

fn main() {
    let mut process_command = ProcessCommand::new();
    let output = process_command.execute();
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
}
