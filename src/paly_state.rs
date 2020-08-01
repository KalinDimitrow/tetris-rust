use crate::game_data::*;
use crate::state_machine::*;
use crate::GameResources;
use piston_window::*;
use std::error;

pub struct PlayState {

}

fn drawPlayField(_c : &Context,  _g : &mut G2d, _arguments : &RenderArgs, _device : &mut gfx_device_gl::Device, _resources : &mut GameResources, _data : &GameData) {
    let empty_block = &_resources.empty_block;
    let full_block = &_resources.cube_block;
    let blocks = &_data.play_table;
    let mut position_index : usize = 0;
    blocks.iter().for_each(|block : &PlayBlock| {
        let x = position_index % GAME_FIELD_WIDTH;
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
    let figure = _data.previewFigure();

    let offset = &figure.offset;
    let figure = &figure.figure;
    let sequence = &figure.sequence;
    sequence.iter().for_each(|position : &(i32, i32)| {
        let x = PREVIEW_DEFAULT_POSITION_X + (position.0 * BLOCK_SIZE as i32) as f64 + offset.0;
        let y = PREVIEW_DEFAULT_POSITION_Y + (position.1 * BLOCK_SIZE as i32) as f64 + offset.1;
        image(full_block, _c.transform.trans(x as f64, y as f64), _g);
    });
}

fn drawCurrent(_c : &Context,  _g : &mut G2d, _arguments : &RenderArgs, _device : &mut gfx_device_gl::Device, _resources : &mut GameResources, _data : &GameData) {
    let full_block = &_resources.cube_block;
    let figure = &_data.current_figure;

    let offset = &figure.position;
    let sequence = &figure.sequence;
    sequence.iter().for_each(|position : &(i32, i32)| {
        let x = (position.0 * BLOCK_SIZE as i32) as f64 + offset.0;
        let y = (position.1 * BLOCK_SIZE as i32) as f64 + offset.1;
        image(full_block, _c.transform.trans(x as f64, y as f64), _g);
    });
}

impl PlayState {
    pub fn new() -> Result<Box<dyn State>,Box<dyn error::Error>> {
        Ok(Box::new(PlayState{}))
    }
}

impl State for PlayState {
    fn update(&mut self,  data : &mut GameData, update_args : &UpdateArgs, event : Event) -> StateTransition {
        StateTransition::Hold
    }

    fn handleInput(&mut self, input : Input, time : Option<TimeStamp>, _data : &mut GameData) {

    }

    fn render(&mut self, _c : Context,  _g : &mut G2d, _arguments : &RenderArgs, _device : &mut gfx_device_gl::Device, _resources : &mut GameResources, _data : &GameData){
        clear([1.0; 4], _g);
        let background = &_resources.background;
        image(background, _c.transform, _g);
        drawPlayField(&_c, _g, _arguments, _device, _resources, _data);
        drawScore(&_c, _g, _arguments, _device, _resources, _data);
        drawPreview(&_c, _g, _arguments, _device, _resources, _data);
        drawCurrent(&_c, _g, _arguments, _device, _resources, _data);
    }
}