pub mod cliff_queue;
pub mod process_observer;

use crate::CONFIGS;
use chrono;
use process_observer::ProcessObserver;
pub use process_observer::ProcessObserverTrait;
use std::{fs, io};

pub fn p1_log_on_lower(processes: &ProcessObserver) {
    let lower_limit = CONFIGS.core.lower_limit;
    let file_name = &CONFIGS.policy_configs.p1_log_file;
    let mut violations: Vec<(u32, u64)> = Vec::new();

    for (pid, queue) in processes.iter() {
        if queue.observe_last_n(1)[0] > &lower_limit {
            violations.push((*pid, *queue.observe_last_n(1)[0]));
        }
    }

    if !violations.is_empty() {
        let message = build_string(violations);
        println!("P1 VIOLATIONS{}", message);
        log_to_file(&file_name, &message);
    }
}

pub fn p2_delayed_email_on_upper(processes: &ProcessObserver) {
    let upper_limit = CONFIGS.core.upper_limit;
    let file_name = &CONFIGS.policy_configs.p2_configs.log_file;
    let mut violations: Vec<(u32, u64)> = Vec::new();

    for (pid, queue) in processes.iter() {
        let observations = queue.observe_last_n(5);
        if observations.iter().all(|&x| x > &upper_limit) {
            violations.push((*pid, *queue.observe_last_n(1)[0]));
        }
    }

    if !violations.is_empty() {
        let message = build_string(violations);
        println!("P2 VIOLATIONS{}", message);
        log_to_file(&file_name, &message);
    }
}

pub fn p3_lower_upper_lower_spike_log(processes: &ProcessObserver) {
    let lower_limit = CONFIGS.core.lower_limit;
    let upper_limit = CONFIGS.core.upper_limit;
    let file_name = &CONFIGS.policy_configs.p3_configs.log_file;
    let mut violations: Vec<(u32, u64)> = Vec::new();

    for (pid, queue) in processes.iter() {
        let observations = queue.observe_last_n(3);
        if observations[2] != &0
            && observations[0] < &lower_limit
            && observations[1] > &upper_limit
            && observations[2] < &lower_limit
        {
            violations.push((*pid, *queue.observe_last_n(1)[0]));
        }
    }

    if !violations.is_empty() {
        let message = build_string(violations);
        println!("P3 VIOLATIONS{}", message);
        log_to_file(&file_name, &message);
    }
}

pub fn p4_lower_mid_lower_spike_log(processes: &ProcessObserver) {
    let lower_limit = CONFIGS.core.lower_limit;
    let upper_limit = CONFIGS.core.upper_limit;
    let file_name = &CONFIGS.policy_configs.p4_configs.log_file;
    let mut violations: Vec<(u32, u64)> = Vec::new();

    for (pid, queue) in processes.iter() {
        let observations = queue.observe_last_n(3);
        if observations[2] != &0
            && observations[0] < &lower_limit
            && observations[1] < &upper_limit
            && observations[1] > &lower_limit
            && observations[2] < &lower_limit
        {
            violations.push((*pid, *queue.observe_last_n(1)[0]));
        }
    }

    if !violations.is_empty() {
        let message = build_string(violations);
        println!("P4 VIOLATIONS{}", message);
        log_to_file(&file_name, &message);
    }
}

pub fn p5_lower_upper_mid_spike_log(processes: &ProcessObserver) {
    let lower_limit = CONFIGS.core.lower_limit;
    let upper_limit = CONFIGS.core.upper_limit;
    let file_name = &CONFIGS.policy_configs.p5_configs.log_file;
    let mut violations: Vec<(u32, u64)> = Vec::new();

    for (pid, queue) in processes.iter() {
        let observations = queue.observe_last_n(3);
        if observations[2] != &0
            && observations[0] < &lower_limit
            && observations[1] > &lower_limit
            && observations[1] < &upper_limit
            && observations[2] < &lower_limit
        {
            violations.push((*pid, *queue.observe_last_n(1)[0]));
        }
    }

    if !violations.is_empty() {
        let message = build_string(violations);
        println!("P5 VIOLATIONS{}", message);
        log_to_file(&file_name, &message);
    }
}
