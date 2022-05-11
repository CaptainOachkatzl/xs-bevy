use std::{collections::HashMap, hash::Hash, sync::Mutex};

pub struct FactoryCache<K, V>
where
  K: Eq + Hash + Copy,
{
  cache: HashMap<K, V>,
  lock: Mutex<()>,
  factory_fn: Box<dyn FnMut(K) -> V>,
}

impl<K, V> FactoryCache<K, V>
where
  K: Eq + Hash + Copy,
{
  pub fn new(factory_fn: Box<dyn FnMut(K) -> V>) -> FactoryCache<K, V> {
    FactoryCache {
      cache: HashMap::new(),
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

    self.cache.get(&key).unwrap()
  }
}
