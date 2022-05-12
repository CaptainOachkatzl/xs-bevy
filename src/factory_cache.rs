use std::{collections::{BTreeMap, HashMap}, ops::Index, sync::Mutex, hash::Hash};

pub trait Cache<K, V>: for<'a> Index<&'a K, Output = V>
where
  K: Copy,
{
  fn contains_key(&self, key: &K) -> bool;
  fn insert(&mut self, key: K, value: V);
}

impl<K, V> Cache<K, V> for BTreeMap<K, V>
where
  K: Copy + Ord,
{
  fn contains_key(&self, key: &K) -> bool {
    self.contains_key(key)
  }

  fn insert(&mut self, key: K, value: V) {
    self.insert(key, value);
  }
}

impl<K, V> Cache<K, V> for HashMap<K, V>
where
  K: Copy + Hash + Eq,
{
  fn contains_key(&self, key: &K) -> bool {
    self.contains_key(key)
  }

  fn insert(&mut self, key: K, value: V) {
    self.insert(key, value);
  }
}

pub struct FactoryCache<K, V> {
  cache: Box<dyn Cache<K, V>>,
  lock: Mutex<()>,
  factory_fn: Box<dyn FnMut(K) -> V>,
}

impl<K, V> FactoryCache<K, V>
where
  K: Copy
{
  pub fn new(cache: Box<dyn Cache<K, V>>, factory_fn: Box<dyn FnMut(K) -> V>) -> FactoryCache<K, V> {
    FactoryCache {
      cache,
      lock: Mutex::new(()),
      factory_fn,
    }
  }

  pub fn get(&mut self, key: K) -> &V {
    // early check without mutex
    if self.cache.contains_key(&key) {
      return &self.cache[&key];
    }

    {
      // check with lock if the key is still missing
      let _guard = self.lock.lock().expect("poisoned mutex");
      if !self.cache.contains_key(&key) {
        // insert in locked state
        let val = self.factory_fn.as_mut()(key);
        self.cache.insert(key, val);
      }
      // can leave the lock here because entry is initialized and other thread can just read value
    }

    &self.cache[&key]
  }
}
