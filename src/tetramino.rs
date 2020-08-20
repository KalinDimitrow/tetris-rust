use std::iter::{IntoIterator, Iterator};
use std::ops;

const ROTATION_COUNT: usize = 4;
const TETRAMINO_POINTS_COUNT: usize = 4;
pub const TETRAMINOS_COUNT: usize = 7;

#[derive(Copy, Clone)]
pub enum TetrominoType {
    I = 0,
    O = 1,
    T = 2,
    S = 3,
    Z = 4,
    J = 5,
    L = 6,
    E,
}

pub type TetraminoesData = [TetrominoData; TETRAMINOS_COUNT];

#[derive(Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn add(&self, rhs: &Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Point) -> Self {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl ops::SubAssign for Point {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

pub type TetrominoSequence = [Point; TETRAMINO_POINTS_COUNT];
pub struct TetrominoRotation {
    pub offset: Point,
    pub sequence: TetrominoSequence,
}

impl<'a> IntoIterator for &'a TetrominoRotation {
    type Item = Point;
    type IntoIter = TetrominoRotationIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            index: 0,
            payload: self,
        }
    }
}

pub struct TetrominoRotationIterator<'a> {
    index: usize,
    payload: &'a TetrominoRotation,
}

impl<'a> Iterator for TetrominoRotationIterator<'a> {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < TETRAMINO_POINTS_COUNT {
            let result = self.payload.sequence[self.index].add(&self.payload.offset);
            self.index += 1;
            return Some(result);
        }

        None
    }
}

type TetrominoRotations = [TetrominoRotation; ROTATION_COUNT];

pub struct TetrominoData {
    pub rotations: TetrominoRotations,
    pub termino_type: TetrominoType,
    pub offset: Point,
}

impl TetrominoData {
    pub fn new(rotations: TetrominoRotations, termino_type: TetrominoType) -> Self {
        TetrominoData {
            rotations,
            termino_type,
            offset: Point { x: 0, y: 0 },
        }
    }
}

pub struct Tetramino {
    position: Point,
    tetramino_type_index: usize,
    rotation_index: usize,
}

impl Tetramino {
    pub fn new(tetramino_type_index: usize) -> Tetramino {
        Tetramino {
            position: Point { x: 5, y: 0 },
            tetramino_type_index,
            rotation_index: 0,
        }
    }

    pub fn get_type(&self) -> usize {
        self.tetramino_type_index.clone()
    }

    pub fn get_rotation(&self) -> usize {
        self.rotation_index.clone()
    }

    pub fn get_position(&self) -> &Point {
        &self.position
    }

    pub fn rotate_left(&mut self) {
        self.rotation_index = self.peek_left_rotation();
    }

    pub fn rotate_right(&mut self) {
        self.rotation_index = self.peek_right_rotation();
    }

    pub fn set_rotation(&mut self, rotation: usize) {
        self.rotation_index = rotation;
    }

    pub fn move_it(&mut self, delata: &Point) {
        self.position += *delata;
    }

    pub fn set_position(&mut self, new: Point) {
        self.position = new;
    }

    pub fn peek_right_rotation(&self) -> usize {
        (self.rotation_index + 1) % ROTATION_COUNT
    }

    pub fn peek_left_rotation(&self) -> usize {
        (self.rotation_index + ROTATION_COUNT - 1) % ROTATION_COUNT
    }
}

pub struct RotationCollisionTable {
    tetramino_maping: [usize; TETRAMINOS_COUNT],
    collision_table: [[[Point; 5]; 8]; 3],
}

impl RotationCollisionTable {
    pub fn new() -> RotationCollisionTable {
        let tetramino_maping = [1, 2, 0, 0, 0, 0, 0];
        let collision_table = [
            [
                [
                    Point { x: 0, y: 0 },
                    Point { x: -1, y: 0 },
                    Point { x: -1, y: 1 },
                    Point { x: 0, y: -2 },
                    Point { x: -1, y: -2 },
                ],
                [
                    Point { x: 0, y: 0 },
                    Point { x: 1, y: 0 },
                    Point { x: 1, y: -1 },
                    Point { x: 0, y: 2 },
                    Point { x: 1, y: 2 },
                ],
                [
                    Point { x: 0, y: 0 },
                    Point { x: 1, y: 0 },
                    Point { x: 1, y: 1 },
                    Point { x: 0, y: -2 },
                    Point { x: 1, y: -2 },
                ],
                [
                    Point { x: 0, y: 0 },
                    Point { x: -1, y: 0 },
                    Point { x: -1, y: -1 },
                    Point { x: 0, y: 2 },
                    Point { x: -1, y: 2 },
                ],
                [
                    Point { x: 0, y: 0 },
                    Point { x: 1, y: 0 },
                    Point { x: 1, y: -1 },
                    Point { x: 0, y: 2 },
                    Point { x: 1, y: 2 },
                ],
                [
                    Point { x: 0, y: 0 },
                    Point { x: -1, y: 0 },
                    Point { x: -1, y: 1 },
                    Point { x: 0, y: -2 },
                    Point { x: -1, y: -2 },
                ],
                [
                    Point { x: 0, y: 0 },
                    Point { x: -1, y: 0 },
                    Point { x: -1, y: -1 },
                    Point { x: 0, y: 2 },
                    Point { x: -1, y: 2 },
                ],
                [
                    Point { x: 0, y: 0 },
                    Point { x: 1, y: 0 },
                    Point { x: 1, y: 1 },
                    Point { x: 0, y: -2 },
                    Point { x: 1, y: -2 },
                ],
            ],
            [
                [
                    Point { x: 0, y: 0 },
                    Point { x: -2, y: 0 },
                    Point { x: 1, y: 0 },
                    Point { x: -2, y: -1 },
                    Point { x: 1, y: 2 },
                ],
                [
                    Point { x: 0, y: 0 },
                    Point { x: -1, y: 0 },
                    Point { x: 2, y: 0 },
                    Point { x: -1, y: 2 },
                    Point { x: 2, y: -1 },
                ],
                [
                    Point { x: 0, y: 0 },
                    Point { x: 2, y: 0 },
                    Point { x: -1, y: 0 },
                    Point { x: 2, y: 1 },
                    Point { x: -1, y: -2 },
                ],
                [
                    Point { x: 0, y: 0 },
                    Point { x: 1, y: 0 },
                    Point { x: -2, y: 0 },
                    Point { x: 1, y: -2 },
                    Point { x: -2, y: 1 },
                ],
                [
                    Point { x: 0, y: 0 },
                    Point { x: 2, y: 0 },
                    Point { x: -1, y: 0 },
                    Point { x: 2, y: 1 },
                    Point { x: -1, y: -2 },
                ],
                [
                    Point { x: 0, y: 0 },
                    Point { x: 1, y: 0 },
                    Point { x: -2, y: 0 },
                    Point { x: 1, y: -2 },
                    Point { x: -2, y: 1 },
                ],
                [
                    Point { x: 0, y: 0 },
                    Point { x: -2, y: 0 },
                    Point { x: 1, y: 0 },
                    Point { x: -2, y: -1 },
                    Point { x: 1, y: 2 },
                ],
                [
                    Point { x: 0, y: 0 },
                    Point { x: -1, y: 0 },
                    Point { x: 2, y: 0 },
                    Point { x: -1, y: 2 },
                    Point { x: 2, y: -1 },
                ],
            ],
            [
                [
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: 0 },
                ],
                [
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: 0 },
                ],
                [
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: 0 },
                ],
                [
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: 0 },
                ],
                [
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: 0 },
                ],
                [
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: 0 },
                ],
                [
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: 0 },
                ],
                [
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: 0 },
                ],
            ],
        ];
        RotationCollisionTable {
            tetramino_maping,
            collision_table,
        }
    }

    pub fn collision_sequence(
        &self,
        i: usize,
        j: usize,
        tetramino_type: usize,
    ) -> std::slice::Iter<'_, Point> {
        let rotation_index = RotationCollisionTable::rotation_collision_table_index(i, j);
        let tetramino_index = self.tetramino_maping[tetramino_type as usize];
        self.collision_table[tetramino_index][rotation_index].iter()
    }

    fn rotation_collision_table_index(i: usize, j: usize) -> usize {
        const TABLE: [[usize; 4]; 4] = [[0, 0, 0, 7], [1, 0, 2, 0], [0, 3, 0, 4], [6, 0, 5, 0]];
        TABLE[i][j]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rotation_collision_table_index() {
        assert_eq!(
            RotationCollisionTable::rotation_collision_table_index(0, 1),
            0
        );
        assert_eq!(
            RotationCollisionTable::rotation_collision_table_index(1, 2),
            2
        );
        assert_eq!(
            RotationCollisionTable::rotation_collision_table_index(2, 3),
            4
        );
        assert_eq!(
            RotationCollisionTable::rotation_collision_table_index(3, 0),
            6
        );
        assert_eq!(
            RotationCollisionTable::rotation_collision_table_index(1, 0),
            1
        );
        assert_eq!(
            RotationCollisionTable::rotation_collision_table_index(2, 1),
            3
        );
        assert_eq!(
            RotationCollisionTable::rotation_collision_table_index(3, 2),
            5
        );
        assert_eq!(
            RotationCollisionTable::rotation_collision_table_index(0, 3),
            7
        );
    }
}

pub fn initialize_tetraminoes_data() -> TetraminoesData {
    [
        TetrominoData::new(
            [
                TetrominoRotation {
                    offset: Point { x: 0, y: 0 },
                    sequence: [
                        Point { x: -1, y: 0 },
                        Point { x: 0, y: 0 },
                        Point { x: 1, y: 0 },
                        Point { x: 2, y: 0 },
                    ],
                },
                TetrominoRotation {
                    offset: Point { x: 1, y: 0 },
                    sequence: [
                        Point { x: 0, y: -1 },
                        Point { x: 0, y: 0 },
                        Point { x: 0, y: 1 },
                        Point { x: 0, y: 2 },
                    ],
                },
                TetrominoRotation {
                    offset: Point { x: 1, y: 1 },
                    sequence: [
                        Point { x: 1, y: 0 },
                        Point { x: 0, y: 0 },
                        Point { x: -1, y: 0 },
                        Point { x: -2, y: 0 },
                    ],
                },
                TetrominoRotation {
                    offset: Point { x: 0, y: 1 },
                    sequence: [
                        Point { x: 0, y: 1 },
                        Point { x: 0, y: 0 },
                        Point { x: 0, y: -1 },
                        Point { x: 0, y: -2 },
                    ],
                },
            ],
            TetrominoType::I,
        ),
        TetrominoData::new(
            [
                TetrominoRotation {
                    offset: Point { x: 0, y: 0 },
                    sequence: [
                        Point { x: 0, y: 0 },
                        Point { x: 1, y: 0 },
                        Point { x: 1, y: 1 },
                        Point { x: 0, y: 1 },
                    ],
                },
                TetrominoRotation {
                    offset: Point { x: 0, y: 0 },
                    sequence: [
                        Point { x: 0, y: 0 },
                        Point { x: 1, y: 0 },
                        Point { x: 1, y: 1 },
                        Point { x: 0, y: 1 },
                    ],
                },
                TetrominoRotation {
                    offset: Point { x: 0, y: 0 },
                    sequence: [
                        Point { x: 0, y: 0 },
                        Point { x: 1, y: 0 },
                        Point { x: 1, y: 1 },
                        Point { x: 0, y: 1 },
                    ],
                },
                TetrominoRotation {
                    offset: Point { x: 0, y: 0 },
                    sequence: [
                        Point { x: 0, y: 0 },
                        Point { x: 1, y: 0 },
                        Point { x: 1, y: 1 },
                        Point { x: 0, y: 1 },
                    ],
                },
            ],
            TetrominoType::O,
        ),
        TetrominoData::new(
            [
                TetrominoRotation {
                    offset: Point { x: 0, y: 0 },
                    sequence: [
                        Point { x: -1, y: 0 },
                        Point { x: 0, y: 0 },
                        Point { x: 1, y: 0 },
                        Point { x: 0, y: -1 },
                    ],
                },
                TetrominoRotation {
                    offset: Point { x: 0, y: 0 },
                    sequence: [
                        Point { x: 0, y: -1 },
                        Point { x: 0, y: 0 },
                        Point { x: 0, y: 1 },
                        Point { x: 1, y: 0 },
                    ],
                },
                TetrominoRotation {
                    offset: Point { x: 0, y: 0 },
                    sequence: [
                        Point { x: 1, y: 0 },
                        Point { x: 0, y: 0 },
                        Point { x: -1, y: 0 },
                        Point { x: 0, y: 1 },
                    ],
                },
                TetrominoRotation {
                    offset: Point { x: 0, y: 0 },
                    sequence: [
                        Point { x: 0, y: 1 },
                        Point { x: 0, y: 0 },
                        Point { x: 0, y: -1 },
                        Point { x: -1, y: 0 },
                    ],
                },
            ],
            TetrominoType::T,
        ),
        TetrominoData::new(
            [
                TetrominoRotation {
                    offset: Point { x: 0, y: 0 },
                    sequence: [
                        Point { x: -1, y: 0 },
                        Point { x: 0, y: 0 },
                        Point { x: 0, y: -1 },
                        Point { x: 1, y: -1 },
                    ],
                },
                TetrominoRotation {
                    offset: Point { x: 0, y: 0 },
                    sequence: [
                        Point { x: 0, y: -1 },
                        Point { x: 0, y: 0 },
                        Point { x: 1, y: 0 },
                        Point { x: 1, y: 1 },
                    ],
                },
                TetrominoRotation {
                    offset: Point { x: 0, y: 0 },
                    sequence: [
                        Point { x: 1, y: 0 },
                        Point { x: 0, y: 0 },
                        Point { x: 0, y: 1 },
                        Point { x: -1, y: 1 },
                    ],
                },
                TetrominoRotation {
                    offset: Point { x: 0, y: 0 },
                    sequence: [
                        Point { x: 0, y: 1 },
                        Point { x: 0, y: 0 },
                        Point { x: -1, y: 0 },
                        Point { x: -1, y: -1 },
                    ],
                },
            ],
            TetrominoType::S,
        ),
        TetrominoData::new(
            [
                TetrominoRotation {
                    offset: Point { x: 0, y: 0 },
                    sequence: [
                        Point { x: -1, y: -1 },
                        Point { x: 0, y: -1 },
                        Point { x: 0, y: 0 },
                        Point { x: 1, y: 0 },
                    ],
                },
                TetrominoRotation {
                    offset: Point { x: 0, y: 0 },
                    sequence: [
                        Point { x: 1, y: -1 },
                        Point { x: 1, y: 0 },
                        Point { x: 0, y: 0 },
                        Point { x: 0, y: 1 },
                    ],
                },
                TetrominoRotation {
                    offset: Point { x: 0, y: 0 },
                    sequence: [
                        Point { x: 1, y: 1 },
                        Point { x: 0, y: 1 },
                        Point { x: 0, y: 0 },
                        Point { x: -1, y: 0 },
                    ],
                },
                TetrominoRotation {
                    offset: Point { x: 0, y: 0 },
                    sequence: [
                        Point { x: -1, y: 1 },
                        Point { x: -1, y: 0 },
                        Point { x: 0, y: 0 },
                        Point { x: 0, y: -1 },
                    ],
                },
            ],
            TetrominoType::Z,
        ),
        TetrominoData::new(
            [
                TetrominoRotation {
                    offset: Point { x: 0, y: 0 },
                    sequence: [
                        Point { x: -1, y: -1 },
                        Point { x: -1, y: 0 },
                        Point { x: 0, y: 0 },
                        Point { x: 1, y: 0 },
                    ],
                },
                TetrominoRotation {
                    offset: Point { x: 0, y: 0 },
                    sequence: [
                        Point { x: 1, y: -1 },
                        Point { x: 0, y: -1 },
                        Point { x: 0, y: 0 },
                        Point { x: 0, y: 1 },
                    ],
                },
                TetrominoRotation {
                    offset: Point { x: 0, y: 0 },
                    sequence: [
                        Point { x: 1, y: 1 },
                        Point { x: 1, y: 0 },
                        Point { x: 0, y: 0 },
                        Point { x: -1, y: 0 },
                    ],
                },
                TetrominoRotation {
                    offset: Point { x: 0, y: 0 },
                    sequence: [
                        Point { x: -1, y: 1 },
                        Point { x: 0, y: 1 },
                        Point { x: 0, y: 0 },
                        Point { x: 0, y: -1 },
                    ],
                },
            ],
            TetrominoType::J,
        ),
        TetrominoData::new(
            [
                TetrominoRotation {
                    offset: Point { x: 0, y: 0 },
                    sequence: [
                        Point { x: -1, y: 0 },
                        Point { x: 0, y: 0 },
                        Point { x: 1, y: 0 },
                        Point { x: 1, y: -1 },
                    ],
                },
                TetrominoRotation {
                    offset: Point { x: 0, y: 0 },
                    sequence: [
                        Point { x: 0, y: -1 },
                        Point { x: 0, y: 0 },
                        Point { x: 0, y: 1 },
                        Point { x: 1, y: 1 },
                    ],
                },
                TetrominoRotation {
                    offset: Point { x: 0, y: 0 },
                    sequence: [
                        Point { x: 1, y: 0 },
                        Point { x: 0, y: 0 },
                        Point { x: -1, y: 0 },
                        Point { x: -1, y: 1 },
                    ],
                },
                TetrominoRotation {
                    offset: Point { x: 0, y: 0 },
                    sequence: [
                        Point { x: 0, y: 1 },
                        Point { x: 0, y: 0 },
                        Point { x: 0, y: -1 },
                        Point { x: -1, y: -1 },
                    ],
                },
            ],
            TetrominoType::L,
        ),
    ]
}
