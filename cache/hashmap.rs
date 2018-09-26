use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use serde_cbor::Value;
use serde_cbor::{from_value, to_value};

use Cache;

/// A cache implemented with a HashMap.
pub struct HashMapCache {
    inner: HashMap<String, Value>,
}

impl Cache for HashMapCache {
    type Error = ();

    fn get<K, V>(&self, key: K) -> Result<V, Self::Error>
    where
        K: AsRef<str>,
        for<'a> V: Deserialize<'a>,
    {
        self.inner
            .get(key.as_ref())
            .and_then(|value| from_value(value.clone()).ok())
            .ok_or(())
    }

    fn set<K, V>(&mut self, key: K, value: V) -> Result<(), Self::Error>
    where
        K: AsRef<str>,
        V: Serialize,
    {
        self.inner
            .insert(key.as_ref().to_owned(), to_value(value).map_err(|_| ())?);
        Ok(())
    }
}
