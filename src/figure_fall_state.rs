use crate::game_resources::*;
use crate::state_machine::*;
use crate::paly_state::*;
use crate::game_data::*;
use piston_window::*;
use std::error;

const TIME_INTERVAL : f64 = 0.33;

pub struct FallingState {
    passed_time : f64
}

impl FallingState {
    pub fn new() -> Result<Box<dyn State>,Box<dyn error::Error>> {
        Ok(Box::new(FallingState{passed_time : 0.0}))
    }
}

impl State for FallingState {
    fn update(&mut self,  data : &mut GameData, update_args : &UpdateArgs, event : Event) -> StateTransition {
        self.passed_time += update_args.dt;
        if self.passed_time >= TIME_INTERVAL {
            self.passed_time -= TIME_INTERVAL;
            if checkForCollision(&(data.current_figure.position.0, data.current_figure.position.1 + 1), &data.current_figure.sequence, &data) {
                for element in &data.current_figure.sequence {
                    let new_position = (data.current_figure.position.0 + element.0, data.current_figure.position.1 + element.1);
                    let index = new_position.0 as usize + (new_position.1 as usize) *WIDTH;
                    data.play_table[index] = PlayBlock::O;
                }
                data.current_figure = GameData::unsafeConvert(data.next_figure).figure.clone();
                data.nextFigure();
                // data.current_figure.sequence.iter().for_each(|element|{
                //     let new_position = (data.current_figure.position.0 + element.0, data.current_figure.position.1 + element.1);
                //     data.play_table[new_position.0 as usize + (new_position.1 as usize) *WIDTH] = PlayBlock::O;
                // })
            } else {
                data.current_figure.position.1 += 1;
            }
        }
        StateTransition::Hold
    }

    fn handleInput(&mut self, input : Input, time : Option<TimeStamp>, _data : &mut GameData) {

    }

    fn backGroundRender(&mut self, c : Context,  g : &mut G2d, arguments : &RenderArgs, device : &mut gfx_device_gl::Device, resources : &mut GameResources, data : &GameData){

    }

    fn render(&mut self, c : Context,  g : &mut G2d, arguments : &RenderArgs, device : &mut gfx_device_gl::Device, resources : &mut GameResources, data : &GameData) {

    }

    fn enter(&mut self, stateMachine : &mut StateMachine) {

    }

    fn exit(&mut self, stateMachine : &mut StateMachine) {

    }
}