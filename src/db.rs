use diesel::{self, prelude::*, r2d2::{ConnectionManager,Pool}};

pub struct DbExecutor<T: Connection + 'static>(pub Pool<ConnectionManager<T>>);

