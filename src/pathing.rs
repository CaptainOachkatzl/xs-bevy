use pathfinding::prelude::astar;

use crate::{patterns::*, *};

pub fn path_exists<'a, T: Copy>(grid: &Grid<T>, start: Position, end: Position, is_pathable_tile: &dyn Fn(T) -> bool) -> bool {
    get_shortest_path(grid, start, end, is_pathable_tile).is_some()
}

pub fn get_shortest_path<'a, T: Copy>(
    grid: &Grid<T>,
    start: Position,
    end: Position,
    is_pathable_tile: &dyn Fn(T) -> bool,
) -> Option<(Vec<Position>, i64)> {
    let cost = |node: &Position| {
        let distance = end - *node;
        return distance.x.abs() + distance.y.abs();
    };
    astar(
        &start,
        |node| get_neighbors(grid, *node, is_pathable_tile),
        cost,
        |node| *node == end,
    )
}

fn get_neighbors<T: Copy>(grid: &Grid<T>, node: Position, is_pathable_tile: &dyn Fn(T) -> bool) -> Vec<(Position, i64)> {
    adjacent_pattern()
        .get_pattern_positions(node)
        .iter()
        .filter_map(|&pos| {
            let entity = grid.get_value_by_position(pos)?;
            if is_pathable_tile(entity) {
                Some((pos, 1))
            } else {
                None
            }
        })
        .collect()
}
