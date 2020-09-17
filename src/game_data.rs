use crate::tetramino::Point;
use crate::tetramino::*;
use rand::prelude::*;
use std::error;

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 20;

pub type GameField = [TetrominoType; WIDTH * HEIGHT];

pub struct GameData {
    pub running: bool,
    pub score: u32,
    pub next_figure: usize,
    pub current_figure: Tetramino,
    pub tetraminoes_data: TetraminoesData,
    pub play_table: GameField,
    pub collision_table: RotationCollisionTable,
    pub highest_level: usize,
    pub lines : usize,
    pub dificulty : u32,
}

impl GameData {
    pub fn new() -> Result<GameData, Box<dyn error::Error>> {
        let play_table = [TetrominoType::E; WIDTH * HEIGHT];
        Ok(GameData {
            running: true,
            score: 0,
            next_figure: GameData::random_tetramino_index(),
            current_figure: Tetramino::new(GameData::random_tetramino_index()),
            tetraminoes_data: initialize_tetraminoes_data(),
            play_table,
            collision_table: RotationCollisionTable::new(),
            highest_level: 0,
            lines : 0,
            dificulty : 0,
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

    pub fn score_multiplier(&self) -> u32 {
        const TABLE : [u32; 3] = [1, 2, 5];
        let index = (self.dificulty % 3) as usize;
        let power = self.dificulty / 3;
        TABLE[index]*(10 as u32).pow(power)*100
    }

    pub fn add_score(&mut self, score : u32) {
        self.score += score;
        self.dificulty = ((self.score as f64 / 500 as f64).ln()).trunc() as u32;
    }

    pub fn speed_multiplier(&self) -> f64 {
        (1.2 as f64).powi(self.dificulty as i32)
    }
}
