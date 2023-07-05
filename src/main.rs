use std::process::Command;

fn build_command() -> Command {
    let mut command = Command::new("ps");
    command.args(&["-e", "-o", "pid,drs", "--sort", "-drs"]);

    command
}

fn main() {
    let mut command = build_command();
    let output = command.output().expect("failed to execute process");
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
}
