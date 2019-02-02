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

type GetFuture<'store, V> = Box<Future<Item = Option<&'store V>, Error = errors::Error> + 'store>;

type StoreFuture<V> = Box<Future<Item = V, Error = errors::Error>>;

/// Key-value storage api interface.
pub trait KVStore<K, V>: KVStoreGet<K, V> + KVStorePut<K, V> + KVStoreRemove<K> {}

pub trait KVStoreGet<K, V> {
    /// Gets stored value for specified key.
    fn get(&self, key: &K) -> GetFuture<V>;
}

pub trait KVStorePut<K, V> {
    /// Puts key value pair (K, V). Update existing value if it's present.
    fn put(&mut self, key: K, value: V) -> StoreFuture<()>;
}

pub trait KVStoreRemove<K> {
    /// Removes pair (K, V) for specified key.
    fn remove(&mut self, key: &K) -> StoreFuture<()>;
}

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

// todo implement Iterable isn't impossible yet, let's wait GAT or HKT in Rust

// todo use std::Futures instead futures::Future
