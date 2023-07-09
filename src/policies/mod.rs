pub mod cliff_queue;
pub mod process_observer;

use chrono;
use process_observer::ProcessObserver;
pub use process_observer::ProcessObserverTrait;
use std::{fs, io};

fn log_to_file(file_name: &str, message: &str) {
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

fn build_string(violations: Vec<(u32, u64)>) -> String {
    let mut message = String::new();
    let time = chrono::Utc::now();

    message.push_str(&format!("\n[UTC TIMESTAMP: {}]\n", time));

    for (pid, cpu_usage) in violations {
        message.push_str(&format!("PID: {}\tMEM: {}\n", pid, cpu_usage));
    }
    message
}

pub fn p1_log_on_lower(processes: &ProcessObserver, lower_limit: u64) {
    let mut violations: Vec<(u32, u64)> = Vec::new();

    for (pid, queue) in processes.iter() {
        if queue.observe_last_n(1)[0] > &lower_limit {
            violations.push((*pid, *queue.observe_last_n(1)[0]));
        }
    }

    if !violations.is_empty() {
        let message = build_string(violations);
        println!("{}", message);
        log_to_file("p1.log", &message);
    }
}
