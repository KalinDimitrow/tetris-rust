use crate::tetramino::Point;
use crate::tetramino::*;
use rand::prelude::*;
use std::error;

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 20;
pub const TETRAMINOS_COUNT: usize = 7;

#[derive(Copy, Clone)]
pub enum PlayBlock {
    E,
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

type TetraminoesData = [TetrominoData; TETRAMINOS_COUNT];

pub struct GameData {
    pub running: bool,
    pub score: u32,
    pub next_figure: usize,
    pub current_figure: Tetramino,
    pub tetraminoes_data: TetraminoesData,
    pub play_table: [PlayBlock; WIDTH * HEIGHT],
    pub collision_table: RotationCollisionTable,
}

impl GameData {
    pub fn new() -> Result<GameData, Box<dyn error::Error>> {
        let play_table = [PlayBlock::E; WIDTH * HEIGHT];
        Ok(GameData {
            running: true,
            score: 0,
            next_figure: GameData::random_tetramino_index(),
            current_figure: Tetramino::new(GameData::random_tetramino_index()),
            tetraminoes_data: initialize_tetraminoes_data(),
            play_table,
            collision_table: RotationCollisionTable::new(),
        })
    }

    pub fn tetramino_preview_sequence(&self) -> &TetrominoSequence {
        &self.tetraminoes_data[self.next_figure].rotations[0].sequence
    }

    pub fn tetramino_preview_offset(&self) -> &Point {
        &self.tetraminoes_data[self.next_figure].offset
    }

    pub fn tetramino_rotation_offset(&self) -> &Point {
        &self.tetraminoes_data[self.current_figure.get_type()].rotations
            [self.current_figure.get_rotation()]
        .offset
    }

    pub fn random_tetramino_index() -> usize {
        let mut rng = rand::thread_rng();
        let value: usize = rng.gen_range(0, TETRAMINOS_COUNT);
        value
    }
}

fn initialize_tetraminoes_data() -> TetraminoesData {
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
