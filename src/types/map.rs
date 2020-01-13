//! Associated types for `Map`.

use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::FromIterator;
use std::sync::Arc;

/// An internally reference-counted map structure.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Map<K: Hash + Eq, V> {
    inner: Arc<HashMap<K, V>>,
}

impl<K: Hash + Eq, V> Map<K, V> {
    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.inner.get(key)
    }
}

impl<K: Hash + Eq, V> From<HashMap<K, V>> for Map<K, V> {
    fn from(hashmap: HashMap<K, V>) -> Map<K, V> {
        Map {
            inner: Arc::new(hashmap),
        }
    }
}

impl<K: Hash + Eq, V> FromIterator<(K, V)> for Map<K, V> {
    fn from_iter<I>(iter: I) -> Map<K, V>
    where
        I: IntoIterator<Item = (K, V)>,
    {
        iter.into_iter().collect::<HashMap<K, V>>().into()
    }
}

impl<K, V> PartialEq<HashMap<K, V>> for Map<K, V>
where
    K: Hash + Eq,
    V: PartialEq,
{
    fn eq(&self, other: &HashMap<K, V>) -> bool {
        &*self.inner == other
    }
}
