use std::{cell::UnsafeCell, sync::Mutex};

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
///
pub struct FactoryCache<K, V> {
  cache: UnsafeCell<Vec<(K, V)>>,
  lock: Mutex<()>,
  factory_fn: Box<dyn Fn(&K) -> V>,
}

impl<K, V> FactoryCache<K, V>
where
  K: PartialEq,
{
  pub fn new(factory_fn: Box<dyn Fn(&K) -> V>) -> FactoryCache<K, V> {
    FactoryCache {
      cache: UnsafeCell::new(Vec::new()),
      lock: Mutex::new(()),
      factory_fn,
    }
  }

  pub fn get(&self, key: K) -> &V {
    // early check without mutex
    if let Some(val) = self.get_cached(&key) {
      return val;
    }

    // check with lock if the key is still missing
    let _guard = self.lock.lock().expect("poisoned mutex");
    if let Some(val) = self.get_cached(&key) {
      return val;
    } else {
      // insert in locked state
      let val = (self.factory_fn)(&key);
      unsafe {
        // reasons why this is allowed:
        // - cache is fully owned by this struct and can't be accessed from outside
        // - this critical section is the only one to ever access this collection mutable
        // - the insert is append-only -> pointers for old data stays valid

        // reasons why this is needed:
        // - "get" can be looked at as immutable from the outside because it takes &self
        // - this allows multiple read only accesses to the cache in parallel
        let vec = &mut *self.cache.get();
        vec.push((key, val));

        return &vec[vec.len() - 1].1;
      }
    }
  }

  fn get_cached(&self, key: &K) -> Option<&V> {
    let vec = unsafe { &*self.cache.get() };
    let length = vec.len();
    for i in 0..length {
      let (entry_key, value) = &vec[i];
      if *entry_key == *key {
        return Some(value);
      }
    }
    return None;
  }
}
