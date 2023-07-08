pub struct CliffQueue<T> {
    queue: VecDeque<T>,
    max_size: usize,
}

impl<T> CliffQueue<T> {
    pub fn new(max_size: usize) -> Self {
        let queue = VecDeque::with_capacity(max_size);
        Self { queue, max_size }
    }

    pub fn populate(&mut self, items: Vec<T>) {
        for item in items {
            self.push(item);
        }
    }

    pub fn observe_last_n(&self, n: usize) -> Vec<T> {
        let mut items = Vec::new();
        for item in self.queue.iter().rev().take(n) {
            items.push(item.clone());
        }
        items
    }

    pub fn push(&mut self, item: T) {
        if self.queue.len() == self.max_size {
            self.queue.pop_front();
        }
        self.queue.push_back(item);
    }
}
