use crate::states::state_machine::StateTransition::{Pop, Hold};
use crate::states::state_machine::*;
use crate::states::paly::*;
use crate::game_data::*;
use crate::resources::*;
use piston_window::*;
use std::error;

const TIME_INTERVAL: f64 = 0.03;

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
        let time_interval = TIME_INTERVAL / data.speed_multiplier();
        if self.fall_time >= time_interval {
            self.fall_time -= time_interval;
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
        _resources: &mut Resources,
        _data: &GameData,
    ) {
    }

    fn render(
        &mut self,
        c: Context,
        g: &mut G2d,
        arguments: &RenderArgs,
        device: &mut gfx_device_gl::Device,
        resources: &mut Resources,
        data: &GameData,
    ) {
        draw_current(&c, g, arguments, device, resources, data);
    }

    fn enter(&mut self, _data: &mut GameData) {}

    fn exit(&mut self, _data: &mut GameData) {}
}

