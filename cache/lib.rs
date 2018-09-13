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

/// An abstraction for a key-value cache.
pub trait Cache {}
