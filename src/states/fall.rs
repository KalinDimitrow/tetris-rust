use crate::states::state_machine::StateTransition::{Hold, Push, Pop};
use crate::states::line_clearing::*;
use crate::states::state_machine::*;
use crate::states::fast_fall::*;
use crate::states::paly::*;
use crate::tetramino::*;
use crate::resources::*;
use crate::game_data::*;
use piston_window::*;
use std::error;

const TIME_INTERVAL: f64 = 0.33;
const CONTROL_TIME_INTERVAL: f64 = 0.1;
const MOVEMENT_SPEED: i32 = 1;

pub struct FallingState {
    fall_time: f64,
    horizontal_time: f64,
    horizontal_movement: i32,
    left_stroke : bool,
    right_stroke : bool,
    left_pressed: bool,  // piston bug
    right_pressed: bool, // piston bug
    rotate_left: bool,
    rotate_right: bool,
    down_pressed: bool,

}

impl FallingState {
    pub fn new() -> Result<Box<dyn State>, Box<dyn error::Error>> {
        Ok(Box::new(FallingState {
            fall_time: 0.0,
            horizontal_time: 0.0,
            horizontal_movement: 0,
            left_stroke : false,
            right_stroke : false,
            left_pressed: false,
            right_pressed: false,
            rotate_left: false,
            rotate_right: false,
            down_pressed: false,

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
                if current.get_position().y <= 0 {
                    return Pop;
                }

                let position = current.get_position().add(data.tetramino_preview_offset());
                let game_field = &mut data.play_table;
                fill_field(&position, rotation.into_iter(), game_field);
                return Push(LineClearing::new().unwrap());
            } else {
                let current = &mut data.current_figure;
                current.set_position(new_position);
            }
        }

        Hold
    }

    fn handle_rotation(&mut self, data: &mut GameData) {
        let current = &data.current_figure;
        let rotation_index = current.get_rotation();
        let mut next_rotation_index = rotation_index;
        let game_field = &data.play_table;

        if self.rotate_left {
            self.rotate_left = false;
            next_rotation_index = current.peek_left_rotation();
        }

        if self.rotate_right {
            self.rotate_right = false;
            next_rotation_index = current.peek_right_rotation();
        }

        let rotation = &data.tetraminoes_data[current.get_type()].rotations[next_rotation_index];
        let sequence = data.collision_table.collision_sequence(
            rotation_index,
            next_rotation_index,
            current.get_type(),
        );

        let mut collision = true;
        let mut free_position = Point { x: 0, y: 0 };
        for point in sequence {
            let new_position = current.get_position().add(point);
            if !check_for_collision(&new_position, rotation, game_field) {
                collision = false;
                free_position = *point;
                break;
            }
        }

        if collision {
            return;
        }

        let current = &mut data.current_figure;
        current.move_it(&free_position);
        current.set_rotation(next_rotation_index);
    }

    fn handle_horizontal_movement(&mut self, dt: f64, data: &mut GameData) {
        self.horizontal_time += dt;
        let time_interval = CONTROL_TIME_INTERVAL;// / data.speed_multiplier();
        if self.horizontal_time >= time_interval {
            self.horizontal_time -= time_interval;
            let current = &data.current_figure;
            let rotation =
                &data.tetraminoes_data[current.get_type()].rotations[current.get_rotation()];
            let mut new_position = current.get_position().clone();

            if self.horizontal_movement == 0 {
                if self.left_stroke {
                    new_position.x -= MOVEMENT_SPEED;
                }

                if self.right_stroke {
                    new_position.x += MOVEMENT_SPEED;
                }
            } else {
                new_position.x += self.horizontal_movement;
            }

            self.left_stroke = false;
            self.right_stroke = false;
            let game_field = &data.play_table;
            if !check_for_collision(&new_position, rotation, game_field) {
                data.current_figure.set_position(new_position);
            }
        }
    }
}

impl State for FallingState {
    fn update(
        &mut self,
        data: &mut GameData,
        update_args: &UpdateArgs,
        _event: Event,
    ) -> StateTransition {
        let state = self.handle_fall(update_args.dt, data);
        match state {
            StateTransition::Hold => {}
            _=> {
              return state;
            }
        }

        if self.down_pressed {
            self.down_pressed = false;
            return StateTransition::Push(FastFallingState::new().unwrap());
        }

        self.handle_horizontal_movement(update_args.dt, data);
        self.handle_rotation(data);
        StateTransition::Hold
    }

    fn handle_input(&mut self, input: Input, _time: Option<TimeStamp>, _data: &mut GameData) {
        match input {
            Input::Button(buttons) => match buttons.button {
                Button::Keyboard(key) => match key {
                    Key::Left => {
                        if buttons.state == ButtonState::Press {
                            if !self.left_pressed {
                                self.horizontal_movement -= MOVEMENT_SPEED;
                                self.left_stroke = true;
                                self.left_pressed = true;
                            }
                        } else {
                            self.horizontal_movement = 0;
                            self.left_pressed = false;
                        }
                    }
                    Key::Right => {
                        if buttons.state == ButtonState::Press {
                            if !self.right_pressed {
                                self.horizontal_movement += MOVEMENT_SPEED;
                                self.right_pressed = true;
                                self.right_stroke = true;
                            }
                        } else {
                            self.horizontal_movement = 0;
                            self.right_pressed = false;
                        }
                    }
                    Key::Up => {
                        if buttons.state == ButtonState::Press {
                            self.rotate_left = true;
                        }
                    }

                    Key::Down => {
                        if buttons.state == ButtonState::Press {
                            self.rotate_right = true;
                        }
                    }

                    Key::Space => {
                        if buttons.state == ButtonState::Press {
                            self.down_pressed = true;
                        }
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }

    }

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
    fn exit(&mut self, data: &mut GameData) {}

    fn resume(&mut self, _data: &mut GameData) {
        self.horizontal_movement = 0;
        self.right_stroke = false;
        self.left_stroke = false;
    }
}
