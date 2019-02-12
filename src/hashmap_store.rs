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
    V: Sync + Send + Clone + 'static,
{
    fn get<'a: 's, 's>(&'s self, key: &K) -> GetFuture<'a, V> {
        let result: Option<V> = self.data.get(key).cloned();
        Box::new(futures::finished(result))
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
    K: Hash + Eq + Send + Sync,
    V: Send + Sync + Clone + 'static,
{
}

#[cfg(test)]
mod tests {
    use crate::hashmap_store::HashMapStore;
    use crate::*;
    use futures::prelude::*;
    use futures_locks::RwLock;
    use std::rc::Rc;
    use std::sync::Arc;

    #[test]
    fn send_test() {
        let _store: Box<Send + Sync> = Box::new(HashMapStore::<i32, String>::new());
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
        assert_eq!(Some(32), store.get(&"key").wait().unwrap());
    }

    #[test]
    fn remove_from_empty_store() {
        let mut store = HashMapStore::<&str, i32>::new();
        assert_eq!((), store.remove(&"key").wait().unwrap());
        assert_eq!(None, store.get(&"key").wait().unwrap());
    }

    #[test]
    fn functional_test() {
        let mut store: Box<KVStore<&str, i32>> = Box::new(HashMapStore::<&str, i32>::new());
        assert_eq!(None, store.get(&"k1").wait().unwrap());
        assert_eq!((), store.put("k1", 10).wait().unwrap());
        assert_eq!(Some(10), store.get(&"k1").wait().unwrap());
        assert_eq!((), store.put("k1", 10).wait().unwrap());
        assert_eq!(Some(10), store.get(&"k1").wait().unwrap());
        assert_eq!((), store.put("k1", 11).wait().unwrap());
        assert_eq!(Some(11), store.get(&"k1").wait().unwrap());
        assert_eq!((), store.remove(&"k1").wait().unwrap());
        assert_eq!(None, store.get(&"k1").wait().unwrap());

        assert_eq!((), store.put("k2", 20).wait().unwrap());
        assert_eq!((), store.put("k3", 30).wait().unwrap());
        assert_eq!((), store.put("k4", 40).wait().unwrap());
        assert_eq!((), store.remove(&"k2").wait().unwrap());
        assert_eq!(Some(30), store.get(&"k3").wait().unwrap());
    }

    #[test]
    fn rwlock_test() {
        // checks compatibility with futures_locks::RwLock
        let store: RwLock<Box<dyn KVStore<usize, String>>> =
            RwLock::new(Box::new(HashMapStore::new()));

        let empty_get = store
            .read()
            .map_err(|_| "Can't get read access".into())
            .and_then(|kv| kv.get(&1));

        let put = store
            .write()
            .map_err(|_| "Can't get read access".into())
            .and_then(|mut kv| kv.put(1, "first".into()));

        let get = store
            .read()
            .map_err(|_| "Can't get read access".into())
            .and_then(|kv| kv.get(&1));

        assert_eq!(None, empty_get.wait().unwrap());
        assert_eq!((), put.wait().unwrap());
        assert_eq!(Some("first".into()), get.wait().unwrap());
    }

    // todo more tests
}
