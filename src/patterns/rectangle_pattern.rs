use std::{collections::BTreeMap, lazy::SyncLazy, sync::Arc};

use crate::{index_translation::to_index, *};

use super::*;

pub fn rectangle_pattern(offset_left: usize, offset_up: usize, offset_right: usize, offset_down: usize) -> Arc<GridPattern> {
  static mut PATTERN_CACHE: SyncLazy<
    FactoryCache<(usize, usize, usize, usize), GridPattern, BTreeMap<(usize, usize, usize, usize), Arc<GridPattern>>>,
  > = SyncLazy::new(|| FactoryCache::new(BTreeMap::new(), Box::new(|(l, u, r, d)| new_rectangle_pattern(l, u, r, d))));

  unsafe { PATTERN_CACHE.get((offset_left, offset_up, offset_right, offset_down)) }
}

pub fn new_rectangle_pattern(offset_left: usize, offset_up: usize, offset_right: usize, offset_down: usize) -> GridPattern {
  let center = Position::from((offset_left, offset_up));
  let width = offset_left + 1 + offset_right;
  let height = offset_up + 1 + offset_down;

  let mut mapping = vec![true; width * height];
  mapping[to_index(center, Size2D::new(width, height))] = false;
  GridPattern {
    mapping: Grid::new(width, height, mapping.into_boxed_slice()),
    center,
  }
}
