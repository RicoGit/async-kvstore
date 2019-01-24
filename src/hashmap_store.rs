//! In-memory implementation

use crate::KVStore;
use crate::StoreError;
use futures::Future;
use std::collections::HashMap;
use std::hash::Hash;

struct HashMapStore<K: Hash + Eq, V> {
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

    fn get<'store>(
        &'store self,
        key: &K,
    ) -> Box<Future<Item = Option<&'store V>, Error = StoreError> + 'store> {
        Box::new(futures::finished(self.data.get(key)))
    }

    fn put(&mut self, key: K, value: V) -> Box<Future<Item = (), Error = StoreError>> {
        self.data.insert(key, value);
        Box::new(futures::finished(()))
    }

    fn remove(&mut self, key: &K) -> Box<Future<Item = (), Error = StoreError>> {
        self.data.remove(key);
        Box::new(futures::finished(()))
    }

    fn close(self) -> Box<Future<Item = (), Error = StoreError>> {
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
        assert_eq!(Ok(None), result.wait());
    }

    #[test]
    fn put_into_empty_store() {
        let mut store = HashMapStore::<&str, i32>::new();
        assert_eq!(Ok(()), store.put(&"key", 32).wait());
        assert_eq!(Ok(Some(&32)), store.get(&"key").wait());
    }

    #[test]
    fn remove_from_empty_store() {
        let mut store = HashMapStore::<&str, i32>::new();
        assert_eq!(Ok(()), store.remove(&"key").wait());
        assert_eq!(Ok(None), store.get(&"key").wait());
    }

    #[test]
    fn close_empty_store() {
        let store = HashMapStore::<&str, i32>::new();
        assert_eq!(Ok(()), store.close().wait());
    }

    #[test]
    fn functional_test() {
        let mut store = HashMapStore::<&str, i32>::new();
        assert_eq!(Ok(None), store.get(&"k1").wait());
        assert_eq!(Ok(()), store.put("k1", 10).wait());
        assert_eq!(Ok(Some(&10)), store.get(&"k1").wait());
        assert_eq!(Ok(()), store.put("k1", 10).wait());
        assert_eq!(Ok(Some(&10)), store.get(&"k1").wait());
        assert_eq!(Ok(()), store.put("k1", 11).wait());
        assert_eq!(Ok(Some(&11)), store.get(&"k1").wait());
        assert_eq!(Ok(()), store.remove(&"k1").wait());
        assert_eq!(Ok(None), store.get(&"k1").wait());

        assert_eq!(Ok(()), store.put("k2", 20).wait());
        assert_eq!(Ok(()), store.put("k3", 30).wait());
        assert_eq!(Ok(()), store.put("k4", 40).wait());
        assert_eq!(Ok(()), store.remove(&"k2").wait());
        assert_eq!(Ok(Some(&30)), store.get(&"k3").wait());
    }

    // todo more tests
}
