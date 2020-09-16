mod chunk;
mod game_data;
mod main_menu_state;
mod paly_state;
mod state_machine;
mod tetramino;
mod tetramino_fall_state;
mod tetris;

pub use crate::tetris::Tetris;
mod game_resources;
mod fast_fall_state;
mod score_screen_state;
mod pause_state;
mod line_clearing_state;
mod chunk_falling_state;

pub use crate::game_resources::GameResources;
extern crate rand;
