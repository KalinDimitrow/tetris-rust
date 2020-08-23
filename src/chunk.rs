use crate::tetramino::Point;

pub struct Chunk {
    position: Point,
    elements: Vec<Point>,
}

impl Chunk {
    pub fn new(data: Vec<Point>) -> Self {
        Chunk {
            position: Point { x: 0, y: 0 },
            elements: Vec::new(),
        }
    }

    pub fn iterator(&self) -> std::slice::Iter<Point> {
        self.elements.iter()
    }
}
