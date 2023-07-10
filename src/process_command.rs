use std::{
    collections::HashMap,
    process::{Command, Output},
};

pub struct ProcessCommand {
    command: Command,
    lower_limit: u64,
}

impl ProcessCommand {
    pub fn new(lower_limit: u64) -> Self {
        let mut command = Command::new("ps");

        #[cfg(target_os = "macos")]
        command.args(&["-e", "-o", "pid,rss", "-m"]);

        #[cfg(target_os = "linux")]
        command.args(&["-e", "-o", "pid,drs", "--sort", "-drs"]);

        Self {
            command,
            lower_limit,
        }
    }

    pub fn execute(&mut self) -> Output {
        self.command.output().expect("failed to execute process")
    }

    pub fn convert_output_to_map(&mut self) -> HashMap<u32, u64> {
        let output = self.execute();
        let output = String::from_utf8_lossy(&output.stdout);
        let mut map = HashMap::new();

        // Skip the first line, which is the header.
        let mut output = output.lines();
        output.next();

        for line in output {
            let mut iter = line.split_whitespace();
            let pid = iter.next().unwrap().parse::<u32>().unwrap();
            let drs = iter.next().unwrap().parse::<u64>().unwrap();

            map.insert(pid, drs);
        }

        map
    }
}
