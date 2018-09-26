use redis::{Client, Commands, FromRedisValue, RedisError};
use serde::{Deserialize, Serialize};
use serde_cbor::{from_slice, to_vec};

use Cache;

/// A cache backed by Redis.
#[derive(Clone)]
pub struct RedisCache {
    client: Client,
}

impl Cache for RedisCache {
    type Error = ();

    fn get<K, V>(&self, key: K) -> Result<V, Self::Error>
    where
        K: AsRef<str>,
        for<'a> V: Deserialize<'a>,
    {
        self.client
            .get(key.as_ref())
            .and_then(|value: redis::Value| Vec::<u8>::from_redis_value(&value))
            .map_err(|_| ())
            .and_then(|vec| from_slice(vec.as_slice()).map_err(|_| ()))
    }

    fn set<K, V>(&mut self, key: K, value: V) -> Result<(), Self::Error>
    where
        K: AsRef<str>,
        V: Serialize,
    {
        let value = to_vec(&value).map_err(|_| ())?;
        self.client.set::<_, _, ()>(key.as_ref(), value);
        Ok(())
    }
}

impl RedisCache {
    /// Creates a new cache using a Redis connection URL.
    pub fn new(connection: impl AsRef<str>) -> Result<Self, RedisError> {
        let client = Client::open(connection.as_ref())?;
        Ok(RedisCache { client })
    }
}
