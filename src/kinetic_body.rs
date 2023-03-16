use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Mass(pub f32);

// pub struct Rectangle {
//     pub left: f32,
//     pub top: f32,
//     pub right: f32,
//     pub bottom: f32,
// }

// impl Rectangle {
//     pub fn contains(&self, other: &Rectangle) -> bool {
//         self.left <= other.left
//             && self.bottom <= other.bottom
//             && self.right >= other.right
//             && self.top >= other.top
//     }
// }

#[derive(Component)]
pub enum CollisionFrame {
    //Rectangle(Vec2),
    Circle(f32),
}

impl Default for CollisionFrame {
    fn default() -> Self {
        Self::Circle(default())
    }
}

impl CollisionFrame {
    pub fn collision(&self, transform_self: &Transform, other: &CollisionFrame, transform_other: &Transform) -> bool {
        match (self, other) {
            (Self::Circle(radius_self), Self::Circle(radius_other)) => collision_circles(
                *radius_self,
                transform_self.translation.truncate(),
                *radius_other,
                transform_other.translation.truncate(),
            ),
        }
    }
}

fn collision_circles(radius1: f32, center1: Vec2, radius2: f32, center2: Vec2) -> bool {
    let distance = center1.distance(center2);
    distance < radius1 + radius2
}

// fn collision_rectangles(rec1: &Rectangle, rec2: &Rectangle) -> bool {
//     rec1.left < rec2.right
//         && rec1.right > rec2.left
//         && rec1.bottom < rec2.top
//         && rec1.top > rec2.bottom
// }
