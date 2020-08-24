use crate::chunk::*;
use crate::game_data::*;
use crate::game_resources::*;
use crate::paly_state::*;
use crate::state_machine::*;
use crate::tetramino::*;
use piston_window::*;
use std::error;

const TIME_INTERVAL: f64 = 0.33;
const CONTROL_TIME_INTERVAL: f64 = 0.1;
const MOVEMENT_SPEED: i32 = 1;

pub struct FallingState {
    fall_time: f64,
    horizontal_time: f64,
    horizontal_movement: i32,
    left_pressed: bool,  // piston bug
    right_pressed: bool, // piston bug
    rotate_left: bool,
    rotate_right: bool,
}

impl FallingState {
    pub fn new() -> Result<Box<dyn State>, Box<dyn error::Error>> {
        Ok(Box::new(FallingState {
            fall_time: 0.0,
            horizontal_time: 0.0,
            horizontal_movement: 0,
            left_pressed: false,
            right_pressed: false,
            rotate_left: false,
            rotate_right: false,
        }))
    }

    fn handle_fall(&mut self, dt: f64, data: &mut GameData) {
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
                let position = current.get_position().add(data.tetramino_preview_offset());
                for element in rotation.into_iter() {
                    let element_position = position.add(&element);
                    let index = element_position.x as usize + (element_position.y as usize) * WIDTH;
                    data.play_table[index] = TetrominoType::O;
                }
                score(data);
                data.current_figure = Tetramino::new(data.next_figure);
                data.next_figure = GameData::random_tetramino_index();
            } else {
                let current = &mut data.current_figure;
                current.set_position(new_position);
            }
        }
    }

    fn handle_rotation(&mut self, data: &mut GameData) {
        let current = &data.current_figure;
        let rotation_index = current.get_rotation();
        let mut next_rotation_index = rotation_index;
        let game_field = &data.play_table;

        if self.rotate_left {
            self.rotate_left = false;
            next_rotation_index = current.peek_left_rotation();

            // data.current_figure.rotate_left();
        }

        if self.rotate_right {
            self.rotate_right = false;
            next_rotation_index = current.peek_right_rotation();
            // data.current_figure.rotate_right();
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
        if self.horizontal_time >= CONTROL_TIME_INTERVAL {
            self.horizontal_time -= CONTROL_TIME_INTERVAL;
            let current = &data.current_figure;
            let rotation =
                &data.tetraminoes_data[current.get_type()].rotations[current.get_rotation()];
            let mut new_position = current.get_position().clone();

            new_position.x += self.horizontal_movement;
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
        self.handle_fall(update_args.dt, data);
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
                                self.left_pressed = true;
                            }
                        } else {
                            self.horizontal_movement += MOVEMENT_SPEED;
                            self.left_pressed = false;
                        }
                    }
                    Key::Right => {
                        if buttons.state == ButtonState::Press {
                            if !self.right_pressed {
                                self.horizontal_movement += MOVEMENT_SPEED;
                                self.right_pressed = true;
                            }
                        } else {
                            self.horizontal_movement -= MOVEMENT_SPEED;
                            self.right_pressed = false;
                        }
                    }
                    Key::A => {
                        if buttons.state == ButtonState::Press {
                            self.rotate_left = true;
                        }
                    }

                    Key::D => {
                        if buttons.state == ButtonState::Press {
                            self.rotate_right = true;
                        }
                    }

                    Key::Down => {}
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
