pub mod cliff_queue;
pub mod process_observer;

mod utils;

use crate::email;
use crate::CONFIGS;
use process_observer::ProcessObserver;
pub use process_observer::ProcessObserverTrait;
use utils::{build_string, log_to_file};

fn p1_log_on_lower(processes: &ProcessObserver) {
    if !CONFIGS.policy_configs.policy_1.enabled {
        return;
    }

    let lower_limit = CONFIGS.core.lower_limit;
    let file_name = &CONFIGS.policy_configs.policy_1.log_file;
    let mut violations: Vec<(u32, Vec<&u64>)> = Vec::new();

    for (pid, queue) in processes.iter() {
        let memory = queue.observe_last_n(1);
        if memory[0] > &lower_limit {
            violations.push((*pid, memory));
        }
    }

    if !violations.is_empty() {
        let message = build_string(violations);
        println!("P1 VIOLATIONS{}", message);
        log_to_file(&file_name, &message);
    }
}

fn p2_delayed_email_on_upper(processes: &ProcessObserver) {
    if !CONFIGS.policy_configs.policy_2.enabled {
        return;
    }

    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let susbject = format!("Policy 2 Violation [{} UTC]", timestamp);
    let message =
        "Policy 2 violations have been detected. Please check the log file for more details.";
    let attachment = email::AttachmentInfo::new(
        &CONFIGS.policy_configs.policy_2.log_file,
        "application/toml; charset=utf-8",
    );

    let upper_limit = CONFIGS.core.upper_limit;
    let delay = CONFIGS.policy_configs.policy_2.delay;
    let file_name = &CONFIGS.policy_configs.policy_2.log_file;
    let mut violations: Vec<(u32, Vec<&u64>)> = Vec::new();

    for (pid, queue) in processes.iter() {
        let observations = queue.observe_last_n(delay);
        if observations.iter().all(|&x| x > &upper_limit) {
            violations.push((*pid, queue.observe_last_n(delay)));
        }
    }

    if !violations.is_empty() {
        let violations_string = build_string(violations);
        let message = format!("{}\n{}", message, violations_string);
        println!("P2 VIOLATIONS{}", message);
        log_to_file(&file_name, &message);
        if CONFIGS.policy_configs.policy_2.enable_email {
            email::send_email(susbject.as_str(), message.as_str(), Some(attachment));
        }
    }
}

fn p3_lower_upper_lower_spike_log(processes: &ProcessObserver) {
    if !CONFIGS.policy_configs.policy_3.enabled {
        return;
    }

    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let susbject = format!("Policy 3 Violation [{} UTC]", timestamp);
    let message =
        "Policy 3 violations have been detected. Please check the log file for more details.";
    let attachment = email::AttachmentInfo::new(
        &CONFIGS.policy_configs.policy_3.log_file,
        "application/toml; charset=utf-8",
    );

    let lower_limit = CONFIGS.core.lower_limit;
    let upper_limit = CONFIGS.core.upper_limit;
    let file_name = &CONFIGS.policy_configs.policy_3.log_file;
    let mut violations: Vec<(u32, Vec<&u64>)> = Vec::new();

    for (pid, queue) in processes.iter() {
        let observations = queue.observe_last_n(3);
        if observations[2] != &0
            && observations[0] < &lower_limit
            && observations[1] > &upper_limit
            && observations[2] < &lower_limit
        {
            violations.push((*pid, queue.observe_last_n(3)));
        }
    }

    if !violations.is_empty() {
        let violations_string = build_string(violations);
        let message = format!("{}\n{}", message, violations_string);
        println!("P3 VIOLATIONS{}", message);
        log_to_file(&file_name, &message);
        if CONFIGS.policy_configs.policy_3.enable_email {
            email::send_email(susbject.as_str(), message.as_str(), Some(attachment));
        }
    }
}

fn p4_lower_mid_lower_spike_log(processes: &ProcessObserver) {
    if !CONFIGS.policy_configs.policy_4.enabled {
        return;
    }

    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let susbject = format!("Policy 4 Violation [{} UTC]", timestamp);
    let message =
        "Policy 4 violations have been detected. Please check the log file for more details.";
    let attachment = email::AttachmentInfo::new(
        &CONFIGS.policy_configs.policy_4.log_file,
        "application/toml; charset=utf-8",
    );

    let lower_limit = CONFIGS.core.lower_limit;
    let upper_limit = CONFIGS.core.upper_limit;
    let file_name = &CONFIGS.policy_configs.policy_4.log_file;
    let mut violations: Vec<(u32, Vec<&u64>)> = Vec::new();

    for (pid, queue) in processes.iter() {
        let observations = queue.observe_last_n(3);
        if observations[2] != &0
            && observations[0] < &lower_limit
            && observations[1] < &upper_limit
            && observations[1] > &lower_limit
            && observations[2] < &lower_limit
        {
            violations.push((*pid, queue.observe_last_n(3)));
        }
    }

    if !violations.is_empty() {
        let violations_string = build_string(violations);
        let message = format!("{}\n{}", message, violations_string);
        println!("P4 VIOLATIONS{}", message);
        log_to_file(&file_name, &message);
        if CONFIGS.policy_configs.policy_4.enable_email {
            email::send_email(susbject.as_str(), message.as_str(), Some(attachment));
        }
    }
}

fn p5_lower_upper_mid_spike_log(processes: &ProcessObserver) {
    if !CONFIGS.policy_configs.policy_5.enabled {
        return;
    }

    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let susbject = format!("Policy 5 Violation [{} UTC]", timestamp);
    let message =
        "Policy 5 violations have been detected. Please check the log file for more details.";
    let attachment = email::AttachmentInfo::new(
        &CONFIGS.policy_configs.policy_5.log_file,
        "application/toml; charset=utf-8",
    );

    let lower_limit = CONFIGS.core.lower_limit;
    let upper_limit = CONFIGS.core.upper_limit;
    let file_name = &CONFIGS.policy_configs.policy_5.log_file;
    let mut violations: Vec<(u32, Vec<&u64>)> = Vec::new();

    for (pid, queue) in processes.iter() {
        let observations = queue.observe_last_n(3);
        if observations[2] != &0
            && observations[0] < &lower_limit
            && observations[1] > &lower_limit
            && observations[1] < &upper_limit
            && observations[2] < &lower_limit
        {
            violations.push((*pid, queue.observe_last_n(3)));
        }
    }

    if !violations.is_empty() {
        let violations_string = build_string(violations);
        let message = format!("{}\n{}", message, violations_string);
        println!("P5 VIOLATIONS{}", message);
        log_to_file(&file_name, &message);
        if CONFIGS.policy_configs.policy_5.enable_email {
            email::send_email(susbject.as_str(), message.as_str(), Some(attachment));
        }
    }
}

pub fn run_policies(processes: &ProcessObserver) {
    p1_log_on_lower(processes);
    p2_delayed_email_on_upper(processes);
    p3_lower_upper_lower_spike_log(processes);
    p4_lower_mid_lower_spike_log(processes);
    p5_lower_upper_mid_spike_log(processes);
}
