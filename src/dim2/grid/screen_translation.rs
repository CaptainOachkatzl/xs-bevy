#![allow(dead_code)]

use crate::{dim2::grid::{Position, RectSize}, ScreenView};

/// converts points inside a grid into their screen coordinates
#[derive(Clone, Copy)]
pub struct ScreenTranslation {
    screen_view: ScreenView,
    grid_size: RectSize,
    tile_width: f32,
    tile_height: f32,
}

impl ScreenTranslation {
    pub fn new(screen_view: ScreenView, grid_size: RectSize) -> ScreenTranslation {
        ScreenTranslation {
            screen_view,
            grid_size,
            tile_width: screen_view.width / grid_size.width as f32,
            tile_height: screen_view.height / grid_size.height as f32,
        }
    }

    pub fn get_grid_position_x(&self, screen_x: f32) -> Option<i64> {
        if !self.in_screen_bounds_horizontally(screen_x) {
            return None;
        }
        let logic_x = (screen_x - self.screen_view.offset_x) / self.tile_width;
        Some(logic_x as i64)
    }

    pub fn get_grid_position_y(&self, screen_y: f32) -> Option<i64> {
        if !self.in_screen_bounds_vertically(screen_y) {
            return None;
        }
        let logic_y = (screen_y - self.screen_view.offset_y) / self.tile_height;
        Some(logic_y as i64)
    }

    fn in_screen_bounds_horizontally(&self, screen_x: f32) -> bool {
        (self.screen_view.offset_x..self.screen_view.offset_x + self.screen_view.width).contains(&screen_x)
    }

    fn in_screen_bounds_vertically(&self, screen_y: f32) -> bool {
        (self.screen_view.offset_y..self.screen_view.offset_y + self.screen_view.height).contains(&screen_y)
    }

    pub fn get_grid_position(&self, screen_x: f32, screen_y: f32) -> Option<Position> {
        let x = self.get_grid_position_x(screen_x)?;
        let y = self.get_grid_position_y(screen_y)?;

        Some(Position { x, y })
    }

    pub fn block_center_to_screen_position(&self, x: i64, y: i64) -> (f32, f32) {
        (self.horizontal_center_to_pixel(x), self.vertical_center_to_pixel(y))
    }

    pub fn horizontal_center_to_pixel(&self, x: i64) -> f32 {
        self.screen_view.offset_x + (x as f32 * self.tile_width) + (self.tile_width / 2.)
    }

    pub fn vertical_center_to_pixel(&self, y: i64) -> f32 {
        self.screen_view.offset_y + (y as f32 * self.tile_height) + (self.tile_height / 2.)
    }
}
