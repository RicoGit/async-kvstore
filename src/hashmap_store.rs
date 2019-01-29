//! In-memory implementation

use crate::errors::*;
use crate::GetFuture;
use crate::KVStore;
use crate::StoreFuture;
use futures::Future;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Default, Debug)]
pub struct HashMapStore<K: Hash + Eq, V> {
    data: HashMap<K, V>,
}

impl<K: Hash + Eq, V> HashMapStore<K, V> {
    pub fn new() -> Self {
        HashMapStore {
            data: HashMap::new(),
        }
    }
}

impl<K: Hash + Eq, V> KVStore<K, V> for HashMapStore<K, V> {
    fn get(&self, key: &K) -> GetFuture<V> {
        Box::new(futures::finished(self.data.get(key)))
    }

    fn put(&mut self, key: K, value: V) -> StoreFuture<()> {
        self.data.insert(key, value);
        Box::new(futures::finished(()))
    }

    fn remove(&mut self, key: &K) -> StoreFuture<()> {
        self.data.remove(key);
        Box::new(futures::finished(()))
    }

    fn close(self) -> StoreFuture<()> {
        Box::new(futures::finished(()))
    }
}

#[cfg(test)]
mod tests {
    use crate::hashmap_store::HashMapStore;
    use crate::KVStore;
    use futures::prelude::*;

    #[test]
    fn get_from_empty_store() {
        let store = HashMapStore::<&str, i32>::new();
        let result = store.get(&"key");
        assert_eq!(None, result.wait().unwrap());
    }

    #[test]
    fn put_into_empty_store() {
        let mut store = HashMapStore::<&str, i32>::new();
        assert_eq!((), store.put(&"key", 32).wait().unwrap());
        assert_eq!(Some(&32), store.get(&"key").wait().unwrap());
    }

    #[test]
    fn remove_from_empty_store() {
        let mut store = HashMapStore::<&str, i32>::new();
        assert_eq!((), store.remove(&"key").wait().unwrap());
        assert_eq!(None, store.get(&"key").wait().unwrap());
    }

    #[test]
    fn close_empty_store() {
        let store = HashMapStore::<&str, i32>::new();
        assert_eq!((), store.close().wait().unwrap());
    }

    #[test]
    fn functional_test() {
        let mut store = HashMapStore::<&str, i32>::new();
        assert_eq!(None, store.get(&"k1").wait().unwrap());
        assert_eq!((), store.put("k1", 10).wait().unwrap());
        assert_eq!(Some(&10), store.get(&"k1").wait().unwrap());
        assert_eq!((), store.put("k1", 10).wait().unwrap());
        assert_eq!(Some(&10), store.get(&"k1").wait().unwrap());
        assert_eq!((), store.put("k1", 11).wait().unwrap());
        assert_eq!(Some(&11), store.get(&"k1").wait().unwrap());
        assert_eq!((), store.remove(&"k1").wait().unwrap());
        assert_eq!(None, store.get(&"k1").wait().unwrap());

        assert_eq!((), store.put("k2", 20).wait().unwrap());
        assert_eq!((), store.put("k3", 30).wait().unwrap());
        assert_eq!((), store.put("k4", 40).wait().unwrap());
        assert_eq!((), store.remove(&"k2").wait().unwrap());
        assert_eq!(Some(&30), store.get(&"k3").wait().unwrap());
    }

    // todo more tests
}
