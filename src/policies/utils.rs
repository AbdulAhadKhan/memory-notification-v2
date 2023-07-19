use chrono;
use std::{fs, io};

pub fn log_to_file(file_name: &str, message: &str) {
    use fs::OpenOptions;
    use io::Write;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_name)
        .unwrap();

    if let Err(e) = writeln!(file, "{}", message) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

pub fn build_string(violations: Vec<(u32, Vec<&u64>)>) -> String {
    let mut message = String::new();
    let time = chrono::Utc::now();

    message.push_str(&format!("\n[UTC TIMESTAMP: {}]\n", time));

    for (pid, memory_usage) in violations {
        let mut memory = String::new();
        for mem in memory_usage {
            memory.push_str(&format!("\t{: <8}", mem));
        }
        message.push_str(&format!("PID: {: <8}\tMEM: {}\n", pid, memory));
    }
    message
}
