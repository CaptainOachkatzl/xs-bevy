use std::collections::BTreeMap;

use xs_bevy_core_2d::{Counter, FactoryCache};

#[test]
fn validate_results() {
  const TEST_CYCLES: i64 = 100;
  let cache = FactoryCache::new(BTreeMap::new(), Box::new(generate_square));

  // check original results
  for i in 0..TEST_CYCLES {
    assert!(*cache.get(i) == generate_square(&i));
  }

  // check cached results
  for i in 0..TEST_CYCLES {
    assert!(*cache.get(i) == generate_square(&i));
  }
}

fn generate_square(x: &i64) -> i64 {
  x * x
}

#[test]
fn confirm_cached() {
  static COUNTER: Counter = Counter::new();

  let call_counter = |_: &usize| {
    COUNTER.tick();
    return 1;
  };

  let cache = FactoryCache::new(BTreeMap::new(), Box::new(call_counter));

  assert_eq!(COUNTER.count(), 0);
  cache.get(1);
  assert_eq!(COUNTER.count(), 1);
  cache.get(1);
  assert_eq!(COUNTER.count(), 1);
}
