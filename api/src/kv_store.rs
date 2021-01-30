//! Traits for async KVStore

use std::future::Future;
use std::pin::Pin;

pub type Task<'a, Val> = Pin<Box<dyn Future<Output = Val> + Send + 'a>>;

/// Aggregated trait for KVStore
pub trait KVStore<K: Send, V: Send>: GetOp<K, V> + SetOp<K, V> {}

/// Get value by key
pub trait GetOp<K: Send, V: Send> {
    fn get(&self, key: K) -> Task<Option<V>>;
}

/// Set value by key
pub trait SetOp<K: Send, V: Send> {
    fn set(&mut self, key: K, val: V) -> Task<Option<V>>;
}

// todo add remove, travers (is possible)

/// Auto-derives KVStore for each T that satisfied all requirements.
impl<T, K, V> KVStore<K, V> for T
where
    K: Send,
    V: Send,
    T: GetOp<K, V> + SetOp<K, V>,
{
}
