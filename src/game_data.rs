use crate::game_logic::GameLogic;
use rand::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::error;

pub const WIDTH : usize = 10;
pub const HEIGHT : usize = 20;

#[derive(Copy, Clone, Rand)]
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
    pub running : bool,
    pub score : u32,
    pub next_figure : PlayBlock,
    pub current_figure : PlayBlock,
    pub play_table : [PlayBlock ; WIDTH*HEIGHT]
}

impl GameData {
    pub fn new() -> Result<GameData, Box<dyn error::Error>> {
        let _game_logic = GameLogic::new()?;
        let play_table = [PlayBlock::E ; WIDTH*HEIGHT];
        let next_figure: PlayBlock = randomFigure();
        let current_figure: PlayBlock = randomFigure();
        Ok(GameData {
            running : true,
            score : 0,
            next_figure : next_figure,
            current_figure : current_figure,
            play_table : play_table
        })
    }
}

fn randomFigure() -> PlayBlock {
    let mut rng = rand::thread_rng();
    let mut figure: PlayBlock = rng.gen();
    loop {
        match figure {
            PlayBlock::E => {
                figure = rng.gen();
            }
            _=> {
                break;
            }
        }
    }
    figure
}