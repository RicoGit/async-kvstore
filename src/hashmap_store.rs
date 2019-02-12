//! In-memory implementation

use crate::errors::*;
use crate::GetFuture;
use crate::KVStore;
use crate::KVStoreGet;
use crate::KVStorePut;
use crate::KVStoreRemove;
use crate::StoreFuture;
use futures::Future;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Default, Debug)]
pub struct HashMapStore<K, V>
where
    K: Hash + Eq + Send,
    V: Sync + Send,
{
    data: HashMap<K, V>,
}

impl<K, V> HashMapStore<K, V>
where
    K: Hash + Eq + Send,
    V: Sync + Send,
{
    pub fn new() -> Self {
        HashMapStore {
            data: HashMap::new(),
        }
    }
}

impl<K, V> KVStoreGet<K, V> for HashMapStore<K, V>
where
    K: Hash + Eq + Send,
    V: Sync + Send,
{
    fn get(&self, key: &K) -> GetFuture<V> {
        Box::new(futures::finished(self.data.get(key)))
    }
}

impl<K, V> KVStorePut<K, V> for HashMapStore<K, V>
where
    K: Hash + Eq + Send,
    V: Sync + Send,
{
    fn put(&mut self, key: K, value: V) -> StoreFuture<()> {
        self.data.insert(key, value); // todo laziness???
        Box::new(futures::finished(()))
    }
}

impl<K, V> KVStoreRemove<K> for HashMapStore<K, V>
where
    K: Hash + Eq + Send,
    V: Sync + Send,
{
    fn remove(&mut self, key: &K) -> StoreFuture<()> {
        self.data.remove(key);
        Box::new(futures::finished(()))
    }
}

impl<K, V> KVStore<K, V> for HashMapStore<K, V>
where
    K: Hash + Eq + Send,
    V: Sync + Send,
{
}

#[cfg(test)]
mod tests {
    use crate::hashmap_store::HashMapStore;
    use crate::*;
    use futures::prelude::*;
    use std::rc::Rc;
    use std::sync::Arc;

    #[test]
    fn sync_dyn_test() {
        let store: Arc<dyn KVStore<i32, String> + Sync> = Arc::new(HashMapStore::new());
        fn take<T: Send + ?Sized>(_it: Arc<T>) {
            ()
        };
        take(store);
    }

    #[test]
    fn send_test() {
        let _store: Box<Send> = Box::new(HashMapStore::<i32, String>::new());
    }

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
