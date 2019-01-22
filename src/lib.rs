//! KVStore Api

// todo remove
#![allow(unused_imports)]
#![allow(dead_code)]

mod hashmap_store;

#[macro_use]
extern crate error_chain;

use futures::Async;
use futures::Future;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;
use std::marker::PhantomData;

// todo add traverse
// todo use std::Futures instead futures::Future
trait KVStore<K, V> {
    /// Gets stored value for specified key.
    fn get(&self, key: &K) -> GetResult<V>;

    /// Puts key value pair (K, V).
    /// Update existing value if it's present.
    fn put(&mut self, key: K, value: V) -> PutResult;

    /// Removes pair (K, V) for specified key.
    fn remove(&mut self, key: &K) -> RemoveResult;

    /// Release all resources acquired for the storage.
    fn close() -> CloseResult;
}

//
// Get operation
//

struct GetResult<'a, V> {
    value: Option<&'a V>,
}

impl<'a, V> Future for GetResult<'a, V> {
    type Item = Option<&'a V>;
    type Error = StoreError;

    fn poll(&mut self) -> Result<Async<<Self as Future>::Item>, <Self as Future>::Error> {
        Ok(Async::Ready(self.value))
    }
}

//
// Put operation
//

struct PutResult {}

impl Future for PutResult {
    type Item = ();
    type Error = StoreError;

    fn poll(&mut self) -> Result<Async<<Self as Future>::Item>, <Self as Future>::Error> {
        Ok(Async::Ready(()))
    }
}

//
// Remove operation
//

struct RemoveResult {}

impl Future for RemoveResult {
    type Item = ();
    type Error = StoreError;

    fn poll(&mut self) -> Result<Async<<Self as Future>::Item>, <Self as Future>::Error> {
        Ok(Async::Ready(()))
    }
}

//
// Close operation
//

struct CloseResult {}

impl Future for CloseResult {
    type Item = ();
    type Error = StoreError;

    fn poll(&mut self) -> Result<Async<<Self as Future>::Item>, <Self as Future>::Error> {
        Ok(Async::Ready(()))
    }
}

//
// Errors
//

// todo consider to switch to error-chain when it'll be hurts
#[derive(Debug, PartialOrd, PartialEq)]
struct StoreError {
    msg: String,
}

impl Error for StoreError {}

impl Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "StoreError({})", self.msg)
    }
}
