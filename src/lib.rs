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
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;
use std::marker::PhantomData;

use crate::errors::*;

// todo use std::Futures instead futures::Future

type GetFuture<'store, V> = Box<Future<Item = Option<&'store V>, Error = errors::Error> + 'store>;

type StoreFuture<V> = Box<Future<Item = V, Error = errors::Error>>;

/// Key-value storage api interface.
pub trait KVStore<K, V> {
    /// Gets stored value for specified key.
    fn get(&self, key: &K) -> GetFuture<V>;

    /// Puts key value pair (K, V). Update existing value if it's present.
    fn put(&mut self, key: K, value: V) -> StoreFuture<()>;

    /// Removes pair (K, V) for specified key.
    fn remove(&mut self, key: &K) -> StoreFuture<()>;

    /// Release all resources acquired for the storage.
    fn close(self) -> StoreFuture<()>;
}

// todo implement iterators

//
// Errors definition
//
mod errors {
    error_chain! {
        errors {
             StoreError(msg: String) {
                 display("Store Error: {:?}", msg)
             }
         }
    }
}
