pub mod cliff_queue;
pub mod process_observer;

use process_observer::ProcessObserver;
pub use process_observer::ProcessObserverTrait;

pub fn p1_log_on_lower(processes: &ProcessObserver) {
    for (pid, queue) in processes.iter() {
        println!("PID: {}\t P1:\t {:?}", pid, queue.observe_last_n(1));
    }
}
