use crate::game_data::*;
use crate::state_machine::*;
use crate::GameResources;
use crate::game_logic::GameLogic;
use std::cell::RefCell;
use piston_window::*;
use math::Matrix2d;
use std::rc::Rc;
use std::error;

pub struct PlayState {

}

fn drawPlayField(_c : &Context,  _g : &mut G2d, _arguments : &RenderArgs, _device : &mut gfx_device_gl::Device, _resources : &mut GameResources, _data : &GameData) {
    let empty_block = &_resources.empty_block;
    let full_block = &_resources.cube_block;
    let blocks = _data.play_table;
    let mut position_index : usize = 0;
    blocks.iter().for_each(|block : &PlayBlock| {
        let x = (position_index % GAME_FIELD_WIDTH);
        let y = (position_index / GAME_FIELD_WIDTH)*BLOCK_SIZE;
        position_index += BLOCK_SIZE;
        match block {
            PlayBlock::E => {
                image(empty_block, _c.transform.trans(x as f64, y as f64), _g);
            }

            _=> {
                image(full_block, _c.transform.trans(x as f64, y as f64), _g);
            }
        }

    });
}

fn drawScore(_c : &Context,  _g : &mut G2d, _arguments : &RenderArgs, _device : &mut gfx_device_gl::Device, _resources : &mut GameResources, _data : &GameData) {
    let score = _data.score;

    text::Text::new_color([1.0, 1.0, 0.0, 1.0], 32).draw(
        "Score : ",
        &mut _resources.font,
        &_c.draw_state,
        _c.transform.trans(SCORE_TEXT_POSITION_X as f64, SCORE_TEXT_POSITION_Y as f64),
        _g
    ).unwrap();

    text::Text::new_color([1.0, 1.0, 0.0, 1.0], 16).draw(
        &score.to_string(),
        &mut _resources.font,
        &_c.draw_state,
        _c.transform.trans(SCORE_POSITION_X as f64, SCORE_POSITION_Y as f64),
        _g
    ).unwrap();

    _resources.font.factory.encoder.flush(_device);
}

fn drawPreview(_c : &Context,  _g : &mut G2d, _arguments : &RenderArgs, _device : &mut gfx_device_gl::Device, _resources : &mut GameResources, _data : &GameData) {
    let full_block = &_resources.cube_block;
    let figure = _data.next_figure;

    let I_ELEMENT : FigureData = FigureData::new((-2.0*size, -size/2.0),vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)]);
    let O_ELEMENT : FigureData = FigureData::new((-size, -size),vec![(0, 0), (1, 0), (0, 1),(1, 1)]);
    let T_ELEMENT : FigureData = FigureData::new((-1.5*size, -size),vec![(0, 0), (1, 0), (2, 0),(1, 1)]);
    let S_ELEMENT : FigureData = FigureData::new((-size, -1.5*size),vec![(0, 0), (1, 0), (0, 1),(-1, 1)]);
    let Z_ELEMENT : FigureData = FigureData::new((-size, -1.5*size),vec![(0, 0), (1, 0), (1, 1),(2, 1)]);
    let J_ELEMENT : FigureData = FigureData::new((1.0, 1.0),vec![(0, 0), (0, 1), (1, 1),(2, 1)]);
    let L_ELEMENT : FigureData = FigureData::new((1.0, 1.0),vec![(0, 0), (0, 1), (-1, 1),(-2, 1)]);

    let sequence = match figure {
        PlayBlock::I => {&I_ELEMENT}
        PlayBlock::O => {&O_ELEMENT}
        PlayBlock::T => {&T_ELEMENT}
        PlayBlock::S => {&S_ELEMENT}
        PlayBlock::Z => {&Z_ELEMENT}
        PlayBlock::J => {&J_ELEMENT}
        PlayBlock::L => {&L_ELEMENT}
        _ => {&panic!()}
    };

    // let offset = sequence.0;
    let offset = &sequence.position;
    sequence.get_sequence().iter().for_each(|position : &(i32, i32)| {
        let x = PREVIEW_DEFAULT_POSITION_X + (position.0 * BLOCK_SIZE as i32) as f64 + offset.0;
        let y = PREVIEW_DEFAULT_POSITION_Y + (position.1 * BLOCK_SIZE as i32) as f64 + offset.1;
        image(full_block, _c.transform.trans(x as f64, y as f64), _g);
    });
}

// fn sequenceFromFigure() -> ((f64, f64),[(i32,i32);4]) {
//
// }

impl PlayState {
    pub fn new() -> Result<Rc<RefCell<dyn State>>,Box<dyn error::Error>> {
        Ok(Rc::new(RefCell::new(PlayState{})))
    }
}

impl State for PlayState {
    fn update(&mut self,  data : &mut GameData, update_args : &UpdateArgs, event : Event) -> &'static str {
        PLAY_STATE
    }

    fn handleInput(&mut self, input : Input, time : Option<TimeStamp>, _data : &mut GameData) -> &'static str {
        PLAY_STATE
    }

    fn render(&mut self, _c : Context,  _g : &mut G2d, _arguments : &RenderArgs, _device : &mut gfx_device_gl::Device, _resources : &mut GameResources, _data : &GameData){
        clear([1.0; 4], _g);
        let background = &_resources.background;
        image(background, _c.transform, _g);
        drawPlayField(&_c, _g, _arguments, _device, _resources, _data);
        drawScore(&_c, _g, _arguments, _device, _resources, _data);
        drawPreview(&_c, _g, _arguments, _device, _resources, _data)
    }
}