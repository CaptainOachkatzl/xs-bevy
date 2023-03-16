use crate::{Position, Size2D};

#[derive(Copy, Clone)]
pub struct Field {
    pub offset: Position,
    pub size: Size2D,
}
