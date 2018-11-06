use failure::Error;
use futures::{Async, Future, Poll};

use db::Connection;

pub trait Task {}

pub struct TaskWrapper<T> {
    client: TaskClient,
    inner: T,
}

impl<T: Task> TaskWrapper<T> {
    pub fn send(self) -> TaskInstance<T> {
        TaskInstance::new(self.client, self.inner)
    }
}

pub struct TaskInstance<T> {
    client: TaskClient,
    inner: T,
}

impl<T: Task> TaskInstance<T> {
    pub fn new(client: TaskClient, inner: T) -> Self {
        TaskInstance { client, inner }
    }
}

pub struct TaskClient {
    inner: Connection,
}

#[allow(dead_code)]
struct SendEmail {
    priority: u32,
}

impl<T> Future for TaskInstance<T> {
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        Ok(Async::Ready(()))
    }
}
