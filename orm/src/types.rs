pub trait SqlType {}

pub struct TinyInt {}

impl SqlType for TinyInt {}

pub enum SqlTypes {
    TinyInt(TinyInt),
}
