use super::position::Position;

#[derive(Clone, Copy)]
pub struct Size2D {
    pub width: usize,
    pub height: usize,
}

impl Size2D {
    pub fn new(width: usize, height: usize) -> Self {
        Size2D { width, height }
    }

    pub fn get_size(&self) -> usize {
        self.width * self.height
    }

    pub fn iter(&self) -> SizeIter {
        SizeIter {
            size: self,
            next_coords: Position { x: 0, y: 0 },
        }
    }
}

#[derive(Clone, Copy)]
pub struct SizeIter<'a> {
    size: &'a Size2D,
    next_coords: Position,
}

impl<'a> SizeIter<'a> {
    pub fn new(size: &'a Size2D) -> Self {
        SizeIter {
            size,
            next_coords: Position { x: 0, y: 0 },
        }
    }
}

impl<'a> Iterator for SizeIter<'a> {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_coords.y as usize >= self.size.height {
            return None;
        }

        let current_coords = self.next_coords;

        if self.next_coords.x as usize >= self.size.width - 1 {
            self.next_coords.y += 1;
            self.next_coords.x = 0;
        } else {
            self.next_coords.x += 1;
        }

        Some(current_coords)
    }
}
