mod cliff_queue;

use cliff_queue::CliffQueue;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ProcessObserver {
    processes: HashMap<u32, CliffQueue<u64>>,
    window_size: usize,
}

impl ProcessObserver {
    pub fn new(window_size: usize) -> Self {
        let processes = HashMap::new();
        Self {
            processes,
            window_size,
        }
    }

    pub fn update_processes(&mut self, processes: HashMap<u32, u64>) {
        self.processes.retain(|pid, _| processes.contains_key(pid));
        for (pid, cpu_usage) in processes {
            if !self.processes.contains_key(&pid) {
                let mut queue = CliffQueue::new(self.window_size);
                queue.populate(0);
                queue.push(cpu_usage);
                self.processes.insert(pid, queue);
            } else {
                self.processes.get_mut(&pid).unwrap().push(cpu_usage);
            }
        }
    }
}
