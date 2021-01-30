//! Traits for AsyncKVStore with boxed futures ans basic in-memory implementation.

#![allow(dead_code)]

use std::collections::HashMap;

use std::hash::Hash;
use std::sync::{Arc, RwLock};

use futures::future::BoxFuture;
use futures::FutureExt;

//
// API
//

type Task<V> = BoxFuture<'static, V>;

pub trait KVGet<K: Send, V: Send> {
    fn get(&self, key: K) -> Task<Option<V>>;
}

pub trait KVSet<K: Send, V: Send> {
    fn set(&mut self, key: K, val: V) -> Task<Option<V>>;
}

//
// Implementations
//

#[derive(Default, Debug)]
pub struct HashMapKVStorage<K, V>
where
    K: Hash + Eq,
{
    data: Arc<RwLock<HashMap<K, V>>>,
}

impl<K, V> HashMapKVStorage<K, V>
where
    K: Hash + Eq,
{
    pub fn new() -> Self {
        HashMapKVStorage {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl<K, V> KVGet<K, V> for HashMapKVStorage<K, V>
where
    K: Hash + Eq + Send + 'static,
    V: Send + Clone + 'static,
{
    fn get(&self, key: K) -> Task<Option<V>> {
        let res = self
            .data
            .clone()
            .read()
            .map(|lock| lock.get(&key).cloned())
            .ok()
            .flatten();
        async move { res }.boxed()
    }
}

impl<K, V> KVSet<K, V> for HashMapKVStorage<K, V>
where
    K: Hash + Eq + Send + 'static,
    V: Send + Clone + 'static,
{
    fn set(&mut self, key: K, val: V) -> Task<Option<V>> {
        let res = self
            .data
            .clone()
            .write()
            .map(|mut lock| lock.insert(key, val))
            .ok()
            .flatten();
        async move { res }.boxed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn set_and_get() {
        let mut storage = HashMapKVStorage::new();

        assert_eq!(storage.get("test").await, None);
        assert_eq!(storage.set("test", 32).await, None);
        assert_eq!(storage.get("test").await, Some(32));

        assert_eq!(dbg!(storage.set("test", 42).await), Some(32));
        assert_eq!(storage.get("test").await, Some(42));

        assert_eq!(storage.get("test2").await, None);
        assert_eq!(storage.set("test2", 2).await, None);
        assert_eq!(storage.set("test3", 3).await, None);
        assert_eq!(storage.get("test2").await, Some(2));
    }
}
