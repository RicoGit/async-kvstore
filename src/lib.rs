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

pub type GetFuture<'store, V> =
    Box<Future<Item = Option<&'store V>, Error = errors::Error> + Send + 'store>;

pub type StoreFuture<V> = Box<Future<Item = V, Error = errors::Error> + Send>;

/// Key-value storage api interface.
pub trait KVStore<K, V>: KVStoreGet<K, V> + KVStorePut<K, V> + KVStoreRemove<K> + Send
where
    K: Send,
    V: Send + Sync,
{
}

pub trait KVStoreGet<K, V>
where
    K: Send,
    V: Send + Sync,
{
    /// Gets stored value for specified key.
    fn get(&self, key: &K) -> GetFuture<V>;
}

pub trait KVStorePut<K, V>
where
    K: Send,
    V: Send,
{
    /// Puts key value pair (K, V). Update existing value if it's present.
    fn put(&mut self, key: K, value: V) -> StoreFuture<()>;
}

pub trait KVStoreRemove<K>
where
    K: Send,
{
    /// Removes pair (K, V) for specified key.
    fn remove(&mut self, key: &K) -> StoreFuture<()>;
}

//
// Errors definition
//
pub mod errors {
    error_chain! {
        errors {
             StoreError(msg: String) {
                 display("Store Error: {:?}", msg)
             }
         }
    }
}

// todo implementation of Iterable isn't impossible yet, let's wait GAT or HKT in Rust

// todo use std::Futures instead futures::Future
