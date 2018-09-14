use std::collections::BTreeMap;

/// Backend-agnostic schema.
#[derive(Default)]
pub struct Schema {
    _tables: BTreeMap<String, Table>,
}

struct Table {
    _columns: Vec<Column>,
    _indices: Vec<Index>,
}

struct Column {}

struct Index {}

struct _Diff {
}

impl Schema {
    pub fn diff(_schema1: &Schema, _schema2: &Schema) {}
}
