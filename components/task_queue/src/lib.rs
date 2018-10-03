pub trait Task {

}

#[derive(Default)]
pub struct TaskQueue {

}

impl TaskQueue {
    pub fn new() -> Self {
        TaskQueue::default()
    }
}