//! Cache is a small library used for abstracting the cache for the website.
//!
//! Motivation
//! ----------
//!
//! OpenCTF was made with being deployed on a server farm in mind. Thus, the
//! caching backend should be powered by something like Memcached or Redis.
//! Additionally, it'd be nice if testing locally didn't require a connection
//! to such a server.
//!
//! Therefore, this crate provides an interface for the OpenCTF platform to
//! interact with various backends flexibly.

#![deny(missing_docs)]

extern crate serde;
extern crate serde_cbor;

#[cfg(feature = "redis")]
extern crate redis;
#[cfg(feature = "redis")]
mod _redis;

mod hashmap;

use serde::{Deserialize, Serialize};

#[cfg(feature = "redis")]
pub use _redis::RedisCache;
pub use hashmap::HashMapCache;

/// An abstraction for a key-value cache.
pub trait Cache: Clone {
    /// The Error type (should support both `get` and `set`).
    type Error;

    /// Get the value associated with the given `key` from the datastore.
    fn get<K, V>(&self, key: K) -> Result<V, Self::Error>
    where
        K: AsRef<str>,
        for<'a> V: Deserialize<'a>;

    /// Set the value associated with `key` to `value`.
    fn set<K, V>(&mut self, key: K, value: V) -> Result<(), Self::Error>
    where
        K: AsRef<str>,
        V: Serialize;
}
