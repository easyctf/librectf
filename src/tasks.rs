use task_queue::Task;

#[allow(dead_code)]
struct SendEmail {
    priority: u32,
}

impl Task for SendEmail {
    fn get_priority(&self) -> u32 {
        self.priority
    }
}
