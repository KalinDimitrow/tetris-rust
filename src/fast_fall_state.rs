use crate::game_data::*;
use crate::game_resources::*;
use crate::paly_state::*;
use crate::state_machine::*;
use crate::tetramino::*;
use piston_window::*;
use std::error;
use crate::state_machine::StateTransition::{Pop, Hold};

const TIME_INTERVAL: f64 = 0.03;
const MOVEMENT_SPEED: i32 = 1;

pub struct FastFallingState {
    fall_time: f64,
}

impl FastFallingState {
    pub fn new() -> Result<Box<dyn State>, Box<dyn error::Error>> {
        Ok(Box::new(FastFallingState {
            fall_time: 0.0,
        }))
    }

    fn handle_fall(&mut self, dt: f64, data: &mut GameData) -> StateTransition {
        self.fall_time += dt;
        if self.fall_time >= TIME_INTERVAL {
            self.fall_time -= TIME_INTERVAL;
            let current = &data.current_figure;
            let mut new_position = current.get_position().clone();
            new_position.y += 1;
            let rotation =
                &data.tetraminoes_data[current.get_type()].rotations[current.get_rotation()];
            let game_field = &data.play_table;
            if check_for_collision(&new_position, rotation, game_field) {
                return Pop;
            } else {
                let current = &mut data.current_figure;
                current.set_position(new_position);
            }
        }
        Hold
    }
}

impl State for FastFallingState {
    fn update(
        &mut self,
        data: &mut GameData,
        update_args: &UpdateArgs,
        _event: Event,
    ) -> StateTransition {
        self.handle_fall(update_args.dt, data)
    }

    fn handle_input(&mut self, _input: Input, _time: Option<TimeStamp>, _data: &mut GameData) {}

    fn background_render(
        &mut self,
        _c: Context,
        _g: &mut G2d,
        _arguments: &RenderArgs,
        _device: &mut gfx_device_gl::Device,
        _resources: &mut GameResources,
        _data: &GameData,
    ) {
    }

    fn render(
        &mut self,
        _c: Context,
        _g: &mut G2d,
        _arguments: &RenderArgs,
        _device: &mut gfx_device_gl::Device,
        _resources: &mut GameResources,
        _data: &GameData,
    ) {
    }

    fn enter(&mut self, _state_machine: &mut StateMachine) {}

    fn exit(&mut self, _state_machine: &mut StateMachine) {}
}

