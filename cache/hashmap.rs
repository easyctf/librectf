use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};

use serde_cbor::Value;
use serde_cbor::{from_value, to_value};

use Cache;

#[derive(Default)]
struct InnerHashMapCache {
    map: HashMap<String, Value>,
}

impl InnerHashMapCache {
    fn get<K, V>(&self, key: K) -> Result<V, ()>
    where
        K: AsRef<str>,
        for<'a> V: Deserialize<'a>,
    {
        self.map
            .get(key.as_ref())
            .and_then(|value| from_value(value.clone()).ok())
            .ok_or(())
    }

    fn set<K, V>(&mut self, key: K, value: V) -> Result<(), ()>
    where
        K: AsRef<str>,
        V: Serialize,
    {
        self.map
            .insert(key.as_ref().to_owned(), to_value(value).map_err(|_| ())?);
        Ok(())
    }
}

/// A cache implemented with a HashMap.
#[derive(Clone)]
pub struct HashMapCache {
    inner: Arc<RwLock<InnerHashMapCache>>,
}

impl Cache for HashMapCache {
    type Error = ();

    fn get<K, V>(&self, key: K) -> Result<V, Self::Error>
    where
        K: AsRef<str>,
        for<'a> V: Deserialize<'a>,
    {
        self.inner.read().unwrap().get(key)
    }

    fn set<K, V>(&mut self, key: K, value: V) -> Result<(), Self::Error>
    where
        K: AsRef<str>,
        V: Serialize,
    {
        self.inner.write().unwrap().set(key, value)
    }
}

impl HashMapCache {
    /// Creates an empty HashMapCache.
    pub fn new() -> HashMapCache {
        let inner = Arc::new(RwLock::new(InnerHashMapCache::default()));
        HashMapCache { inner }
    }
}
