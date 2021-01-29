//! KVStore Api

#![allow(dead_code)]

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::RwLock;

//
// API
//

#[async_trait]
pub trait KVGet<K: Send + Sync, V: Send + Sync> {
    async fn get(&self, key: K) -> Option<V>;
}

#[async_trait]
pub trait KVSet<K: Send + Sync, V: Send + Sync> {
    async fn set(&mut self, key: K, val: V) -> Option<V>;
}

//
// Implementations
//

#[derive(Default, Debug)]
pub struct HashMapKVStore<K, V>
where
    K: Hash + Eq,
{
    data: Arc<RwLock<HashMap<K, V>>>,
}

impl<K, V> HashMapKVStore<K, V>
where
    K: Hash + Eq,
{
    pub fn new() -> Self {
        HashMapKVStore {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl<K, V> KVGet<K, V> for HashMapKVStore<K, V>
where
    K: Hash + Eq + Send + Sync,
    V: Clone + Send + Sync,
{
    async fn get(&self, key: K) -> Option<V> {
        let map = self.data.clone();
        let lock = map.read().await;
        let res = lock.get(&key).cloned();
        res
    }
}

#[async_trait]
impl<K, V> KVSet<K, V> for HashMapKVStore<K, V>
where
    K: Hash + Eq + Send + Sync,
    V: Clone + Send + Sync,
{
    async fn set(&mut self, key: K, val: V) -> Option<V> {
        let map = self.data.clone();
        let mut lock = map.write().await;
        let res = lock.insert(key, val);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn set_and_get() {
        let mut storage = HashMapKVStore::new();

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
