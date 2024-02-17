use bevy::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct RelativeRectangle {
    pub height: f32,
    pub width: f32,
    /// distance from top_left
    pub offset: Vec2,
}

impl RelativeRectangle {
    fn to_absolute(self, center: Vec2) -> Rectangle {
        let left = center.x - self.offset.x;
        let top = center.y - self.offset.y;
        let right = left + self.width;
        let bottom = top + self.height;
        Rectangle { left, top, right, bottom }
    }
}

struct Rectangle {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl Rectangle {
    pub fn contains_point(&self, point: Vec2) -> bool {
        self.left <= point.x && self.top <= point.y && self.right >= point.x && self.bottom >= point.y
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CollisionFrame {
    Rectangle(RelativeRectangle),
    Circle(f32),
}

impl CollisionFrame {
    pub fn collision(&self, position_self: Vec2, other: &CollisionFrame, position_other: Vec2) -> bool {
        match (self, other) {
            (Self::Circle(radius_self), Self::Circle(radius_other)) => {
                collision_circles(*radius_self, position_self, *radius_other, position_other)
            }
            (Self::Circle(radius), Self::Rectangle(rel_rect)) => {
                collision_circle_with_rect(*radius, position_self, &rel_rect.to_absolute(position_other))
            }
            (Self::Rectangle(rel_rect), Self::Circle(radius)) => {
                collision_circle_with_rect(*radius, position_other, &rel_rect.to_absolute(position_self))
            }
            (Self::Rectangle(rel_rect_self), Self::Rectangle(rel_rect_other)) => collision_rectangles(
                &rel_rect_self.to_absolute(position_self),
                &rel_rect_other.to_absolute(position_other),
            ),
        }
    }

    pub fn is_point_inside(&self, position: Vec2, point: Vec2) -> bool {
        match self {
            Self::Circle(radius) => is_point_in_circle(point, *radius, position),
            Self::Rectangle(rel_rect) => rel_rect.to_absolute(position).contains_point(point),
        }
    }

    pub fn collision_with_any<'a>(&self, position: Vec2, others: impl Iterator<Item = (&'a CollisionFrame, Vec2)>) -> bool {
        for other in others.into_iter() {
            if self.collision(position, other.0, other.1) {
                return true;
            }
        }

        false
    }
}

fn is_point_in_circle(point: Vec2, circle_radius: f32, circle_center: Vec2) -> bool {
    point.distance(circle_center) < circle_radius
}

fn collision_circles(radius1: f32, center1: Vec2, radius2: f32, center2: Vec2) -> bool {
    let distance = center1.distance(center2);
    distance < radius1 + radius2
}

fn collision_rectangles(rec1: &Rectangle, rec2: &Rectangle) -> bool {
    rec1.left < rec2.right && rec1.right > rec2.left && rec1.top < rec2.bottom && rec1.bottom > rec2.top
}

fn collision_circle_with_rect(radius: f32, center: Vec2, rect: &Rectangle) -> bool {
    let mut closest_corner = center;
    // test left edge
    if center.x < rect.left {
        closest_corner.x = rect.left;
    }
    // right edge
    else if center.x > rect.right {
        closest_corner.x = rect.right;
    }
    // top edge
    if center.y < rect.top {
        closest_corner.y = rect.top;
    }
    // bottom edge
    else if center.y > rect.bottom {
        closest_corner.y = rect.bottom;
    }

    is_point_in_circle(closest_corner, radius, center)
}
