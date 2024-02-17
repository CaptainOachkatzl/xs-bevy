use super::{position::Position, size_2d::Size2D};

#[derive(Copy, Clone)]
pub struct Field {
    pub offset: Position,
    pub size: Size2D,
}
