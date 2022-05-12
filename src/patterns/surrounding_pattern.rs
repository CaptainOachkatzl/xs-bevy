use std::{sync::Once, collections::BTreeMap};

use crate::{index_translation::to_index, *};

use super::*;

pub fn surrounding_pattern(arm_length: usize) -> &'static GridPattern {
  static INIT: Once = Once::new();
  static mut PATTERN_CACHE: Option<FactoryCache<usize, GridPattern>> = None;

  unsafe {
    INIT.call_once(|| {
      let factory_fn: Box<dyn Fn(&usize) -> GridPattern> = Box::new(|x| new_surrounding_pattern(*x));
      PATTERN_CACHE = Some(FactoryCache::new(Box::new(BTreeMap::new()), factory_fn));
    });

    PATTERN_CACHE.as_mut().unwrap().get(arm_length)
  }
}

pub fn new_surrounding_pattern(radius: usize) -> GridPattern {
  let center = Position::from((radius, radius));
  let diameter = radius * 2 + 1;
  let mut mapping = vec![true; diameter * diameter];
  mapping[to_index(center, Size2D::new(diameter, diameter))] = false;
  GridPattern {
    mapping: Grid::new(diameter, diameter, mapping.into_boxed_slice()),
    center,
  }
}
