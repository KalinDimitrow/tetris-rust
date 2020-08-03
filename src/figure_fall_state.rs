use crate::game_resources::*;
use crate::state_machine::*;
use crate::paly_state::*;
use crate::game_data::*;
use piston_window::*;
use std::error;
use crate::paly_state::Rotation::{Left, Right};

const TIME_INTERVAL : f64 = 0.33;
const CONTROL_TIME_INTERVAL : f64 = 0.1;
const MOVEMENT_SPEED : i32 = 1;

pub struct FallingState {
    fall_time : f64,
    horizontal_time : f64,
    horizontal_movement : i32,
    left_pressed : bool, // piston bug
    right_pressed : bool, // piston bug
    rotate_left : bool,
    rotate_right : bool,
}

impl FallingState {
    pub fn new() -> Result<Box<dyn State>,Box<dyn error::Error>> {
        Ok(Box::new(FallingState
        {
            fall_time : 0.0,
            horizontal_time : 0.0,
            horizontal_movement : 0,
            left_pressed : false,
            right_pressed : false,
            rotate_left : false,
            rotate_right : false
        }))
    }

    fn handleFall(&mut self, dt : f64, data : &mut GameData) {
        self.fall_time += dt;
        if self.fall_time >= TIME_INTERVAL {
            self.fall_time -= TIME_INTERVAL;
            if checkForCollision(&(data.current_figure.position.0, data.current_figure.position.1 + 1), &data.current_figure.sequence, &data) {
                for element in &data.current_figure.sequence {
                    let new_position = (data.current_figure.position.0 + element.0, data.current_figure.position.1 + element.1);
                    let index = new_position.0 as usize + (new_position.1 as usize) *WIDTH;
                    data.play_table[index] = PlayBlock::O;
                }
                data.current_figure = GameData::unsafeConvert(data.next_figure).figure.clone();
                data.nextFigure();
            } else {
                data.current_figure.position.1 += 1;
            }
        }
    }

    fn handleRotation(&mut self, data : &mut GameData) {
        if self.rotate_left {
            self.rotate_left = false;
            rotate(&mut data.current_figure, Left);
        }

        if self.rotate_right {
            self.rotate_right = false;
            rotate(&mut data.current_figure, Right);
        }
    }

    fn handleHorizontalMovement(&mut self, dt : f64, data : &mut GameData) {
        self.horizontal_time += dt;
        if self.horizontal_time >= CONTROL_TIME_INTERVAL {
            self.horizontal_time -= CONTROL_TIME_INTERVAL;
            if !checkForCollision(&(data.current_figure.position.0 + self.horizontal_movement, data.current_figure.position.1), &data.current_figure.sequence, &data) {
                data.current_figure.position.0 += self.horizontal_movement;
            }
        }
    }
}

impl State for FallingState {
    fn update(&mut self,  data : &mut GameData, update_args : &UpdateArgs, event : Event) -> StateTransition {
        self.handleFall(update_args.dt, data);
        self.handleHorizontalMovement(update_args.dt, data);
        self.handleRotation(data);
        StateTransition::Hold
    }

    fn handleInput(&mut self, input : Input, time : Option<TimeStamp>, _data : &mut GameData) {
        match input {
            Input::Button(buttons) => {
                match buttons.button {
                    Button::Keyboard(key) => {
                        match key {
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

                            Key::Down => {
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            _=>{}
        }
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