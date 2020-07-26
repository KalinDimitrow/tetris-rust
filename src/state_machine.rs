use crate::game_data::*;
use crate::GameResources;
use crate::game_logic::GameLogic;
use crate::figure::FigureData;
use std::cell::RefCell;
use piston_window::*;
use math::Matrix2d;
use std::rc::Rc;
use std::error;
use std::slice;

pub const MAIN_MENU : &str = "Main menu";
pub const PLAY_STATE : &str = "Play state";

pub const BLOCK_SIZE : usize = 64;
pub const GAME_FIELD_WIDTH : usize = BLOCK_SIZE*crate::game_data::WIDTH;

pub const SCORE_TEXT_SIZE : u32 = 32;
pub const SCORE_SIZE : u32 = 16;
pub const SCORE_POSITION_X : f64 = 780.0;
pub const SCORE_POSITION_Y : f64 = 24.0;
pub const SCORE_TEXT_POSITION_X : f64 = 650.0;
pub const SCORE_TEXT_POSITION_Y : f64 = SCORE_TEXT_SIZE as f64;
pub const PREVIEW_DEFAULT_POSITION_X : f64 = 780.0;
pub const PREVIEW_DEFAULT_POSITION_Y : f64 = 240.0;

pub const size : f64 = BLOCK_SIZE as f64;



pub trait State {
    fn update(&mut self,  data : &mut GameData, update_args : &UpdateArgs, event : Event) -> &'static str;
    fn handleInput(&mut self, input : Input, time : Option<TimeStamp>, _data : &mut GameData) -> &'static str;
    fn render(&mut self, _c : Context,  _g : &mut G2d, _arguments : &RenderArgs, _device : &mut gfx_device_gl::Device, _resources : &mut GameResources, _data : &GameData){}
}

pub fn renderText(text : &str, font : &mut Glyphs, transform: Matrix2d, draw_state: &DrawState, g : &mut G2d, selected : bool) {
    let color_pair : (f32,f32) = if selected {
        (0.2, 1.0)
    } else {
        (1.0, 0.2)
    };

    text::Text::new_color([color_pair.0, color_pair.1, 0.0, 1.0], 128).draw(
        text,
        font,
        draw_state,
        transform,
        g
    ).unwrap();
}


