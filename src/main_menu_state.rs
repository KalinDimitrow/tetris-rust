use crate::game_data::*;
use crate::paly_state::PlayState;
use crate::state_machine::*;
use crate::GameResources;
use piston_window::*;
use std::error;

const ELEMENTS_COUNT: i32 = 2;

pub struct MainMenu {
    selection: i32,
    interact: bool,
}

impl MainMenu {
    pub fn new() -> Result<Box<dyn State>, Box<dyn error::Error>> {
        Ok(Box::new(MainMenu {
            selection: 0,
            interact: false,
        }))
    }
}

impl State for MainMenu {
    fn update(
        &mut self,
        _data: &mut GameData,
        _update_args: &UpdateArgs,
        _event: Event,
    ) -> StateTransition {
        if self.interact {
            self.interact = false;
            match self.selection {
                0 => {
                    return StateTransition::Transition(PlayState::new().unwrap());
                }

                1 => {
                    return StateTransition::Pop;
                }

                _ => {
                    panic!();
                }
            }
        }
        StateTransition::Hold
    }

    fn handle_input(&mut self, input: Input, _time: Option<TimeStamp>, _data: &mut GameData) {
        match input {
            Input::Button(buttons) => {
                if buttons.state == ButtonState::Press {
                    match buttons.button {
                        Button::Keyboard(key) => match key {
                            Key::Up => {
                                self.selection =
                                    (self.selection + (ELEMENTS_COUNT - 1)) % ELEMENTS_COUNT;
                            }
                            Key::Down => {
                                self.selection = (self.selection + 1) % ELEMENTS_COUNT;
                            }
                            Key::Return => {
                                self.interact = true;
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
            _ => {}
        }
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
        let texture = &_resources.background;
        let _font = &mut _resources.font;

        clear([1.0; 4], _g);
        image(texture, _c.transform, _g);

        render_text(
            "Start game",
            _font,
            _c.transform.trans(0 as f64, 128 as f64),
            &_c.draw_state,
            _g,
            self.selection == 0,
        );
        render_text(
            "Quit",
            _font,
            _c.transform.trans(0 as f64, 512 as f64),
            &_c.draw_state,
            _g,
            self.selection == 1,
        );
        _font.factory.encoder.flush(_device);
    }
}
