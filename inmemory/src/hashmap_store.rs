//! HashMap-based implementation of [[KVStore]].
//!
//! # Examples
//!
//! ```
//! use kvstore_inmemory::hashmap_store::HashMapKVStore;
//! use kvstore_api::kvstore::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     use kvstore_inmemory::hashmap_store::HashMapKVStore;
//!     let mut store = HashMapKVStore::new();
//!     store.set(1, "test").await;
//!     let res = store.get(1).await.unwrap();
//!     assert_eq!(res, Some("test"))
//! }
//!
//! ```

use kvstore_api::kvstore::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};

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

impl<K, V> GetOp<K, V> for HashMapKVStore<K, V>
where
    K: Hash + Eq + Send,
    V: Send + Clone,
{
    fn get(&self, key: K) -> StoreTask<Option<V>> {
        let res = self
            .data
            .clone()
            .read()
            .map(|lock| lock.get(&key).cloned())
            .ok()
            .flatten();
        Box::pin(async move { Ok(res) })
    }
}

impl<K, V> SetOp<K, V> for HashMapKVStore<K, V>
where
    K: Hash + Eq + Send,
    V: Send + Clone,
{
    fn set(&mut self, key: K, val: V) -> StoreTask<Option<V>> {
        let res = self
            .data
            .clone()
            .write()
            .map(|mut lock| lock.insert(key, val))
            .ok()
            .flatten();
        Box::pin(async move { Ok(res) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn set_and_get() {
        let mut storage = HashMapKVStore::new();

        assert_eq!(storage.get("test").await.unwrap(), None);
        assert_eq!(storage.set("test", 32).await.unwrap(), None);
        assert_eq!(storage.get("test").await.unwrap(), Some(32));

        assert_eq!(dbg!(storage.set("test", 42).await.unwrap()), Some(32));
        assert_eq!(storage.get("test").await.unwrap(), Some(42));

        assert_eq!(storage.get("test2").await.unwrap(), None);
        assert_eq!(storage.set("test2", 2).await.unwrap(), None);
        assert_eq!(storage.set("test3", 3).await.unwrap(), None);
        assert_eq!(storage.get("test2").await.unwrap(), Some(2));
    }
}
