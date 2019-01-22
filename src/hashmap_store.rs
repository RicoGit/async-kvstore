//
// In-memory implementation
//

use crate::CloseResult;
use crate::GetResult;
use crate::KVStore;
use crate::PutResult;
use crate::RemoveResult;
use std::collections::HashMap;
use std::hash::Hash;

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
    fn get(&self, key: &K) -> GetResult<V> {
        GetResult {
            value: self.data.get(key),
        }
    }

    fn put(&mut self, key: K, value: V) -> PutResult {
        self.data.insert(key, value);
        PutResult {}
    }

    fn remove(&mut self, key: &K) -> RemoveResult {
        self.data.remove(key);
        RemoveResult {}
    }

    fn close() -> CloseResult {
        CloseResult {}
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

    // todo more tests
}
