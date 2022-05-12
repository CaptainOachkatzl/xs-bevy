use std::{
  collections::{BTreeMap, HashMap},
  hash::Hash,
  ops::Index,
  sync::Mutex,
};

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

/// # FactoryCache
///
/// Caches the output of a factory/generator function for the given parameter(s).
/// 
/// Repeated calls to "get" are served directly from the cache instead.
///
/// # Examples
///
/// ```
/// fn intensive_calculation(as_f64: usize) -> f64 {
///   thread::sleep(Duration::from_secs(2));
///   as_f64 as f64
/// }
///
/// let factory_cache = FactoryCache::new(Box::new(BTreeMap::new()), Box::new(|x| intensive_calculation(*x)));
/// println!("{}", factory_cache.get(3)); // takes 2 seconds to write "3"
/// println!("{}", factory_cache.get(5)); // takes 2 seconds to write "5"
/// println!("{}", factory_cache.get(3)); // instantly writes "3"
/// ```
pub struct FactoryCache<K, V> {
  cache: Box<dyn Cache<K, V>>,
  lock: Mutex<()>,
  factory_fn: Box<dyn Fn(&K) -> V>,
}

impl<K, V> FactoryCache<K, V>
where
  K: Copy,
{
  pub fn new(cache: Box<dyn Cache<K, V>>, factory_fn: Box<dyn Fn(&K) -> V>) -> FactoryCache<K, V> {
    FactoryCache {
      cache,
      lock: Mutex::new(()),
      factory_fn,
    }
  }

  pub fn get(&self, key: K) -> &V {
    // early check without mutex
    if self.cache.contains_key(&key) {
      return &self.cache[&key];
    }

    {
      // check with lock if the key is still missing
      let _guard = self.lock.lock().expect("poisoned mutex");
      if !self.cache.contains_key(&key) {
        // insert in locked state

        let val = (self.factory_fn)(&key);
        let ptr = self.cache.as_ref() as *const dyn Cache<K, V>;
        unsafe {
          // reasons why this is allowed:
          // - cache is fully owned by this struct and can't be accessed from outside
          // - this critical section is the only one to ever access this collection mutable

          // reasons why this is needed:
          // - "get" can be looked at as immutable from the outside because it takes &self
          // - this allows multiple read only accesses to the cache in parallel
          (ptr as *mut dyn Cache<K, V>).as_mut().unwrap().insert(key, val);
        }
      }
      // can leave the lock here because entry is initialized and other thread can just read value
    }

    &self.cache[&key]
  }
}
