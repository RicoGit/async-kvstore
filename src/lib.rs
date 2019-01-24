//! KVStore Api

// todo remove
#![allow(unused_imports)]
#![allow(dead_code)]

pub mod hashmap_store;

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
pub trait KVStore<K, V> {
    /// Gets stored value for specified key.
    fn get<'store>(
        &'store self,
        key: &K,
    ) -> Box<Future<Item = Option<&'store V>, Error = StoreError> + 'store>;

    /// Puts key value pair (K, V). Update existing value if it's present.
    fn put(&mut self, key: K, value: V) -> Box<Future<Item = (), Error = StoreError>>;

    /// Removes pair (K, V) for specified key.
    fn remove(&mut self, key: &K) -> Box<Future<Item = (), Error = StoreError>>;

    /// Release all resources acquired for the storage.
    fn close(self) -> Box<Future<Item = (), Error = StoreError>>;
}

// todo implement iterators

//
// Errors
//

// todo consider to switch to error-chain when it'll be hurts
#[derive(Debug, PartialOrd, PartialEq)]
pub struct StoreError {
    msg: String,
}

impl Error for StoreError {}

impl Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "StoreError({})", self.msg)
    }
}
