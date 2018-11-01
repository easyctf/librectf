use db::Connection;

pub trait Task {}

pub struct TaskClient {
    inner: Connection,
}

#[allow(dead_code)]
struct SendEmail {
    priority: u32,
}
