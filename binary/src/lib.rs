//! KVStore with binary data representation. Wraps generic KVStore, provides async/await api.
//!
//! # Examples
//!
//! ```
//! use kvstore_inmemory::hashmap_store::HashMapKVStore;
//! use kvstore_api::kvstore::*;
//! use kvstore_binary::BinKVStore;
//!
//! #[tokio::main]
//! async fn main() {
//!
//!     let store = HashMapKVStore::new();
//!     let mut bin_store = BinKVStore::new(store);
//!
//!     bin_store.set("test".to_string(), 42).await.unwrap();
//!     let res = bin_store.get("test".to_string()).await.unwrap();
//!     assert_eq!(res, Some(42))
//! }
//!
//! ```

use std::marker::PhantomData;

use bincode::ErrorKind;
use serde::de::DeserializeOwned;
use serde::Serialize;

use kvstore_api::kvstore::{KVStore, KVStoreError, StoreResult};

#[derive(Debug)]
pub struct BinKVStore<K, V, Store> {
    store: Store,
    _k: PhantomData<K>,
    _v: PhantomData<V>,
}

impl<K, V, Store> BinKVStore<K, V, Store>
where
    K: Serialize + DeserializeOwned + Send,
    V: Serialize + DeserializeOwned + Send,
    Store: KVStore<Vec<u8>, Vec<u8>>,
{
    pub fn new(store: Store) -> Self {
        BinKVStore {
            store,
            _k: PhantomData,
            _v: PhantomData,
        }
    }

    pub async fn get(&self, key: K) -> StoreResult<Option<V>> {
        let bin_key = to_byte(&key)?;
        let bin_result = self.store.get(bin_key).await?;
        let val = bin_result.map(|bytes| from_byte(&bytes)).transpose()?;

        Ok(val)
    }

    pub async fn set(&mut self, key: K, val: V) -> StoreResult<Option<V>> {
        let bin_key = to_byte(&key)?;
        let bin_val = to_byte(&val)?;
        let bin_result = self.store.set(bin_key, bin_val).await?;
        let val = bin_result.map(|bytes| from_byte(&bytes)).transpose()?;

        Ok(val)
    }
}

fn to_byte<T: Serialize>(obj: T) -> StoreResult<Vec<u8>> {
    bincode::serialize(&obj).map_err(from_serde)
}

fn from_byte<T: DeserializeOwned>(bytes: &[u8]) -> StoreResult<T> {
    bincode::deserialize(&(bytes)).map_err(from_serde)
}

/// Maps serde err to KVStoreError, can't impl From trait cause orphan rule.
fn from_serde(err: Box<ErrorKind>) -> KVStoreError {
    KVStoreError {
        msg: format!("Bin serde error: {:?}", err),
    }
}

#[cfg(test)]
mod tests {
    use kvstore_inmemory::hashmap_store::*;

    use super::*;

    #[tokio::test]
    async fn set_and_get() {
        let store = HashMapKVStore::<Vec<u8>, Vec<u8>>::new();
        let mut bin_store = BinKVStore::new(store);

        assert_eq!(bin_store.get("test".to_string()).await.unwrap(), None);
        assert_eq!(bin_store.set("test".to_string(), 32).await.unwrap(), None);
        assert_eq!(bin_store.get("test".to_string()).await.unwrap(), Some(32));

        assert_eq!(
            bin_store.set("test".to_string(), 42).await.unwrap(),
            Some(32)
        );
        assert_eq!(bin_store.get("test".to_string()).await.unwrap(), Some(42));

        assert_eq!(bin_store.get("test2".to_string()).await.unwrap(), None);
        assert_eq!(bin_store.set("test2".to_string(), 2).await.unwrap(), None);
        assert_eq!(bin_store.set("test3".to_string(), 3).await.unwrap(), None);
        assert_eq!(bin_store.get("test2".to_string()).await.unwrap(), Some(2));
    }
}
