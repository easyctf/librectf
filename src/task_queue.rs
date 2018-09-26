use cache::Cache;

#[derive(Serialize, Deserialize)]
pub enum Task {
    SendMail(),
}

pub struct TaskQueue<B: Cache> {
    cache: B,
}

impl<B: Cache> TaskQueue<B> {
    pub fn new(backend: B) -> Self {
        TaskQueue { cache: backend }
    }

    pub fn enqueue(&self, task: Task) {}
}
