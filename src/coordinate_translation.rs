#![allow(dead_code)]

use crate::{field::Field, Position, Size2D};

#[derive(Clone, Copy)]
pub struct CoordinateTranslation {
  pub screen_view: Field,
  pub logical_size: Size2D,
  pub logical_pixel_width: f32,
  pub logical_pixel_height: f32,
}

impl CoordinateTranslation {
  pub fn new(screen_view: Field, logical_size: Size2D) -> CoordinateTranslation {
    return CoordinateTranslation {
      screen_view,
      logical_size,
      logical_pixel_width: screen_view.size.width as f32 / logical_size.width as f32,
      logical_pixel_height: screen_view.size.height as f32 / logical_size.height as f32,
    };
  }

  pub fn get_logical_position_x(&self, screen_x: usize) -> usize {
    let logic_x = (screen_x - self.screen_view.offset.x) as f32 / self.logical_pixel_width;
    return logic_x as usize;
  }

  pub fn get_logical_position_y(&self, screen_y: usize) -> usize {
    let logic_y = (screen_y - self.screen_view.offset.y) as f32 / self.logical_pixel_height;
    return logic_y as usize;
  }

  pub fn get_logical_position(&self, screen_x: usize, screen_y: usize) -> Position {
    Position {
      x: self.get_logical_position_x(screen_x),
      y: self.get_logical_position_y(screen_y),
    }
  }

  pub fn block_center_to_pixel_position(&self, x: usize, y: usize) -> (f32, f32) {
    return (self.horizontal_center_to_pixel(x), self.vertical_center_to_pixel(y));
  }

  pub fn horizontal_center_to_pixel(&self, x: usize) -> f32 {
    return self.screen_view.offset.x as f32 + (x as f32 * self.logical_pixel_width) + (self.logical_pixel_width / 2.);
  }

  pub fn vertical_center_to_pixel(&self, y: usize) -> f32 {
    return self.screen_view.offset.y as f32 + (y as f32 * self.logical_pixel_height) + (self.logical_pixel_height / 2.);
  }
}
