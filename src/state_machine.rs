use crate::game_data::*;
use crate::GameResources;
use piston_window::*;
use math::Matrix2d;
use std::vec::Vec;
use std::error;

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

pub enum StateTransition {
    Push(Box<dyn State>),
    Transition(Box<dyn State>),
    Pop,
    Hold
}

pub trait State {
    fn update(&mut self,  data : &mut GameData, update_args : &UpdateArgs, event : Event) -> StateTransition;
    fn handleInput(&mut self, input : Input, time : Option<TimeStamp>, _data : &mut GameData);
    fn backGroundRender(&mut self, c : Context,  g : &mut G2d, arguments : &RenderArgs, device : &mut gfx_device_gl::Device, resources : &mut GameResources, data : &GameData){}
    fn render(&mut self, c : Context,  g : &mut G2d, arguments : &RenderArgs, device : &mut gfx_device_gl::Device, resources : &mut GameResources, data : &GameData){}
    fn enter(&mut self, stateMachine : &mut StateMachine) {}
    fn exit(&mut self, stateMachine : &mut StateMachine) {}
}

pub struct StateMachine {
    stack : Vec<Box<dyn State>>
}

impl StateMachine {
    pub fn new(initial_state : Box<dyn State>) -> Result<StateMachine, Box<dyn error::Error>> {
        let stack = vec![initial_state];
        Ok(StateMachine{stack})
    }

    pub fn update(&mut self,  data : &mut GameData, update_args : &UpdateArgs, event : Event) -> bool {

        let transition = if let Some(top) = self.stack.last_mut() {
            top.update(data, update_args, event)
        } else {
            StateTransition::Hold
        };

        match transition {
            StateTransition::Push(mut pushed_state) => {
                pushed_state.enter( self);
                self.stack.push(pushed_state);
            }

            StateTransition::Transition(mut transition) => {
                let mut top = self.stack.pop().unwrap();
                top.exit( self);
                transition.enter(self);
                self.stack.push(transition);
            }

            StateTransition::Pop => {
                if let Some(mut top) = self.stack.pop() {
                    top.exit( self);
                }
            }

            StateTransition::Hold => {}
        }

        !self.stack.is_empty()
    }

    pub fn handleInput(&mut self, input : Input, time : Option<TimeStamp>, data : &mut GameData) {
        if let Some(top) = self.stack.last_mut() {
            top.handleInput(input, time, data);
        }
    }

    pub fn render(&mut self, c : Context,  g : &mut G2d, arguments : &RenderArgs, device : &mut gfx_device_gl::Device, resources : &mut GameResources, data : &GameData) {
        // let arguments = arguments;
        self.stack.iter_mut().for_each(|state |{
            state.backGroundRender(c, g, arguments, device, resources, data);
        });
        if let Some(top) = self.stack.last_mut() {
            top.render(c, g, arguments, device, resources, data);
        }
    }
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


