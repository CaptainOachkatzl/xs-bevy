use std::{
  collections::BTreeMap,
  sync::Arc, lazy::SyncLazy,
};

use crate::{index_translation::to_index, *};

use super::*;

pub fn surrounding_pattern(arm_length: usize) -> Arc<GridPattern> {
  static mut PATTERN_CACHE: SyncLazy<FactoryCache<usize, GridPattern, BTreeMap<usize, Arc<GridPattern>>>> = SyncLazy::new(|| {
    FactoryCache::new(BTreeMap::new(), Box::new(new_surrounding_pattern))
  });

  unsafe { PATTERN_CACHE.get(arm_length) }
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
