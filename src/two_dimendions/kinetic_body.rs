#[derive(Debug, Clone, Copy)]
pub struct RelativeRectangle {
    pub height: f32,
    pub width: f32,
    /// distance from top_left
    pub offset_x: f32,
    pub offset_y: f32,
}

impl RelativeRectangle {
    fn to_absolute(self, center_x: f32, center_y: f32) -> Rectangle {
        let left = center_x - self.offset_x;
        let top = center_y - self.offset_y;
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
    pub fn contains_point(&self, point_x: f32, point_y: f32) -> bool {
        self.left <= point_x && self.top <= point_y && self.right >= point_x && self.bottom >= point_y
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CollisionFrame {
    Rectangle(RelativeRectangle),
    Circle(f32),
}

impl CollisionFrame {
    pub fn collision(
        &self,
        position_x: f32,
        position_y: f32,
        other: &CollisionFrame,
        position_other_x: f32,
        position_other_y: f32,
    ) -> bool {
        match (self, other) {
            (Self::Circle(radius_self), Self::Circle(radius_other)) => collision_circles(
                *radius_self,
                position_x,
                position_y,
                *radius_other,
                position_other_x,
                position_other_y,
            ),
            (Self::Circle(radius), Self::Rectangle(rel_rect)) => collision_circle_with_rect(
                *radius,
                position_x,
                position_y,
                &rel_rect.to_absolute(position_other_x, position_other_y),
            ),
            (Self::Rectangle(rel_rect), Self::Circle(radius)) => collision_circle_with_rect(
                *radius,
                position_other_x,
                position_other_y,
                &rel_rect.to_absolute(position_x, position_y),
            ),
            (Self::Rectangle(rel_rect_self), Self::Rectangle(rel_rect_other)) => collision_rectangles(
                &rel_rect_self.to_absolute(position_x, position_y),
                &rel_rect_other.to_absolute(position_other_x, position_other_y),
            ),
        }
    }

    pub fn is_point_inside(&self, position_x: f32, position_y: f32, point_x: f32, point_y: f32) -> bool {
        match self {
            Self::Circle(radius) => is_point_in_circle(point_x, point_y, *radius, position_x, position_y),
            Self::Rectangle(rel_rect) => rel_rect.to_absolute(position_x, position_y).contains_point(point_x, point_y),
        }
    }

    pub fn collision_with_any<'a>(
        &self,
        position_x: f32,
        position_y: f32,
        others: impl Iterator<Item = (&'a CollisionFrame, f32, f32)>,
    ) -> bool {
        for other in others.into_iter() {
            if self.collision(position_x, position_y, other.0, other.1, other.2) {
                return true;
            }
        }

        false
    }
}

fn is_point_in_circle(point_x: f32, point_y: f32, circle_radius: f32, circle_center_x: f32, circle_center_y: f32) -> bool {
    distance(point_x, point_y, circle_center_x, circle_center_y) < circle_radius
}

fn collision_circles(radius1: f32, center1_x: f32, center1_y: f32, radius2: f32, center2_x: f32, center2_y: f32) -> bool {
    let distance = distance(center1_x, center1_y, center2_x, center2_y);
    distance < radius1 + radius2
}

fn collision_rectangles(rec1: &Rectangle, rec2: &Rectangle) -> bool {
    rec1.left < rec2.right && rec1.right > rec2.left && rec1.top < rec2.bottom && rec1.bottom > rec2.top
}

fn collision_circle_with_rect(radius: f32, center_x: f32, center_y: f32, rect: &Rectangle) -> bool {
    let mut closest_corner_x = center_x;
    let mut closest_corner_y = center_y;
    // test left edge
    if center_x < rect.left {
        closest_corner_x = rect.left;
    }
    // right edge
    else if center_x > rect.right {
        closest_corner_x = rect.right;
    }
    // top edge
    if center_y < rect.top {
        closest_corner_y = rect.top;
    }
    // bottom edge
    else if center_y > rect.bottom {
        closest_corner_y = rect.bottom;
    }

    is_point_in_circle(closest_corner_x, closest_corner_y, radius, center_x, center_y)
}

fn distance(x: f32, y: f32, other_x: f32, other_y: f32) -> f32 {
    let dist_x = other_x - x;
    let dist_y = other_y - y;

    (dist_x.powi(2) + dist_y.powi(2)).sqrt()
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case(
        CollisionFrame::Rectangle(RelativeRectangle{ height: 1., width: 1., offset_x: 0., offset_y: 0. }), 0., 0.,
        CollisionFrame::Rectangle(RelativeRectangle{ height: 1., width: 1., offset_x: 0., offset_y: 0. }), 0., 0.,
        true; "complete overlap")]
    #[test_case(
        CollisionFrame::Rectangle(RelativeRectangle{ height: 1., width: 1., offset_x: 0., offset_y: 0. }), 0., 0.,
        CollisionFrame::Rectangle(RelativeRectangle{ height: 1., width: 1., offset_x: 0., offset_y: 0. }), 2., 0.,
        false; "no collision on horizontal line")]
    #[test_case(
        CollisionFrame::Rectangle(RelativeRectangle{ height: 1., width: 1., offset_x: 0., offset_y: 0. }), 0., 0.,
        CollisionFrame::Rectangle(RelativeRectangle{ height: 1., width: 1., offset_x: 0., offset_y: 0. }), 0., 2.,
        false; "no collision on vertical line")]
    #[test_case(
        CollisionFrame::Rectangle(RelativeRectangle{ height: 1., width: 1., offset_x: 0., offset_y: 0. }), 0., 0.,
        CollisionFrame::Rectangle(RelativeRectangle{ height: 1., width: 1., offset_x: 0., offset_y: 0. }), 0.9, 0.9,
        true; "corner collision")]
    fn rectangle_collisions(
        frame1: CollisionFrame,
        center1_x: f32,
        center1_y: f32,
        frame2: CollisionFrame,
        center2_x: f32,
        center2_y: f32,
        expected: bool,
    ) {
        assert_eq!(frame1.collision(center1_x, center1_y, &frame2, center2_x, center2_y), expected);
    }
}
