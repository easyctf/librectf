pub trait Task {
    fn get_priority(&self) -> u32;
}

#[derive(Default)]
pub struct TaskQueue {}

impl TaskQueue {
    pub fn new() -> Self {
        TaskQueue::default()
    }

    pub fn enqueue(&self, task: impl Task) {}
}
