use dashmap::DashMap;
use std::hash::{BuildHasherDefault, Hash};
use std::sync::Arc;
use twox_hash::XxHash64;

type Map<K, H> = DashMap<K, H, BuildHasherDefault<XxHash64>>;

#[derive(Debug)]
pub struct Registry<K, H>
where
    K: Eq + Hash + Clone + 'static,
    H: 'static,
{
    pub map: Arc<Map<K, H>>,
}

impl<K, H> Registry<K, H>
where
    K: Eq + Hash + Clone + 'static,
    H: 'static,
{
    pub fn op<I, O, V>(&self, key: K, op: O, init: I) -> V
    where
        I: FnOnce() -> H,
        O: FnOnce(&H) -> V,
    {
        let valref = self.map.entry(key).or_insert_with(init);
        op(valref.value())
    }
}

impl<K, H> Default for Registry<K, H>
where
    K: Eq + Hash + Clone + 'static,
    H: 'static,
{
    fn default() -> Self {
        Self {
            map: Arc::new(Map::default()),
        }
    }
}

impl<K, H> Clone for Registry<K, H>
where
    K: Eq + Hash + Clone + 'static,
    H: 'static,
{
    fn clone(&self) -> Self {
        Self {
            map: Arc::clone(&self.map),
        }
    }
}
