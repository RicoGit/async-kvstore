//! Traits for async KVStore

use std::future::Future;
use std::pin::Pin;
use thiserror::Error;

/// Aggregated trait for KVStore
pub trait KVStore<K: Send, V: Send>: GetOp<K, V> + SetOp<K, V> {}

#[derive(Error, Debug)]
#[error("KVStore Error: {msg:?}")]
pub struct KVStoreError {
    pub msg: String,
}

pub type StoreResult<V> = std::result::Result<V, KVStoreError>;

pub type StoreTask<'a, Val> = Pin<Box<dyn Future<Output = StoreResult<Val>> + Send + 'a>>;

/// Get value by key
pub trait GetOp<K: Send, V: Send> {
    fn get(&self, key: K) -> StoreTask<Option<V>>;
}

/// Set value by key
pub trait SetOp<K: Send, V: Send> {
    fn set(&mut self, key: K, val: V) -> StoreTask<Option<V>>;
}

// todo add remove, travers (is possible)

/// Auto-derives KVStore for each T that satisfied all requirements.
impl<K, V, Ops> KVStore<K, V> for Ops
where
    K: Send,
    V: Send,
    Ops: GetOp<K, V> + SetOp<K, V>,
{
}
