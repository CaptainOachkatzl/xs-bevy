#![allow(dead_code)]

use crate::{field::Field, Position, Size2D};

#[derive(Clone, Copy)]
pub struct ScreenTranslation {
  pub screen_view: Field,
  pub logical_size: Size2D,
  pub logical_pixel_width: f32,
  pub logical_pixel_height: f32,
}

impl ScreenTranslation {
  pub fn new(screen_view: Field, logical_size: Size2D) -> ScreenTranslation {
    return ScreenTranslation {
      screen_view,
      logical_size,
      logical_pixel_width: screen_view.size.width as f32 / logical_size.width as f32,
      logical_pixel_height: screen_view.size.height as f32 / logical_size.height as f32,
    };
  }

  pub fn get_logical_position_x(&self, screen_x: usize) -> Option<usize> {
    if screen_x < self.screen_view.offset.x || screen_x > self.screen_view.offset.x + self.screen_view.size.width {
      return None;
    }
    let logic_x = (screen_x - self.screen_view.offset.x) as f32 / self.logical_pixel_width;
    return Some(logic_x as usize);
  }

  pub fn get_logical_position_y(&self, screen_y: usize) -> Option<usize> {
    if screen_y < self.screen_view.offset.y || screen_y > self.screen_view.offset.y + self.screen_view.size.height {
      return None;
    }
    let logic_y = (screen_y - self.screen_view.offset.y) as f32 / self.logical_pixel_height;
    return Some(logic_y as usize);
  }

  pub fn get_logical_position(&self, screen_x: usize, screen_y: usize) -> Option<Position> {
    let x = self.get_logical_position_x(screen_x);
    let y = self.get_logical_position_y(screen_y);

    if x.is_none() || y.is_none() {
      return None;
    }
    
    Some(Position {
      x: x.unwrap(),
      y: y.unwrap(),
    })
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
