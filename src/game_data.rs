use crate::figure::{FigureData, PreviewFigureData};
use rand::prelude::*;
use std::error;
use std::vec::Vec;

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 20;

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

pub struct GameData {
    pub running: bool,
    pub score: u32,
    pub next_figure: *const PreviewFigureData,
    pub current_figure: FigureData,
    pub figures: Vec<PreviewFigureData>,
    pub play_table: [PlayBlock; WIDTH * HEIGHT],
}

impl GameData {
    pub fn new() -> Result<GameData, Box<dyn error::Error>> {
        // let _game_logic = GameLogic::new()?;
        let play_table = [PlayBlock::E; WIDTH * HEIGHT];
        let figures = PreviewFigureData::initialize_figures();
        let next_figure = GameData::generate_next_figure(&figures);
        let current_figure: FigureData = GameData::unsafe_convert(next_figure).figure.clone();
        let next_figure = GameData::generate_next_figure(&figures);
        Ok(GameData {
            running: true,
            score: 0,
            next_figure,
            current_figure,
            figures,
            play_table,
        })
    }

    pub fn next_figure(&mut self) {
        self.next_figure = GameData::generate_next_figure(&self.figures);
    }

    pub fn generate_next_figure(figures: &Vec<PreviewFigureData>) -> *const PreviewFigureData {
        figures.choose(&mut rand::thread_rng()).unwrap() as *const PreviewFigureData
    }

    pub fn preview_figure(&self) -> &PreviewFigureData {
        GameData::unsafe_convert(self.next_figure)
    }

    pub fn unsafe_convert<'a>(pointer: *const PreviewFigureData) -> &'a PreviewFigureData {
        let figure = unsafe { &(*(pointer)) };
        figure
    }
}
