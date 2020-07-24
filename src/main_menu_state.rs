use crate::game_data::*;
use crate::state_machine::*;
use crate::GameResources;
use crate::game_logic::GameLogic;
use piston_window::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::error;

pub struct MainMenu {
    selection : i32
}

impl MainMenu {
    pub fn new() -> Result<Rc<RefCell<dyn State>>,Box<dyn error::Error>> {
        Ok(Rc::new(RefCell::new(MainMenu{selection : 0})))
    }
}



impl State for MainMenu {
    fn update(&mut self,  _data : &mut GameData, _update_args : &UpdateArgs, event : Event) -> &'static str {
        MAIN_MENU
    }

    fn handleInput(&mut self, input : Input, time : Option<TimeStamp>, _data : &mut GameData) -> &'static str {
        let max : i32 = 2;
        match input {
            Input::Button(buttons) => {
                if buttons.state == ButtonState::Press {
                    match buttons.button {
                        Button::Keyboard(key) => {
                            match key {
                                Key::Up => {
                                    self.selection = (self.selection + (max - 1)) % max;
                                }
                                Key::Down => {
                                    self.selection = (self.selection + 1) % max;
                                }
                                Key::Return => {
                                    match self.selection {
                                        0 => {
                                            return PLAY_STATE;
                                        }

                                        1 => {
                                            _data.running = false;
                                        }

                                        _=> {
                                            panic!();
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            }
            _=>{}
        }
        MAIN_MENU
    }

    fn render(&mut self, _c : Context, _g : &mut G2d, _arguments : &RenderArgs, _device : &mut gfx_device_gl::Device, _resources : &mut GameResources, _data : &GameData) {
        let texture = &_resources.background;
        let _font = &mut _resources.font;

        clear([1.0; 4], _g);
        image(texture, _c.transform, _g);

        renderText("Start game", _font, _c.transform.trans(0 as f64, 128 as f64), &_c.draw_state, _g, self.selection == 0);
        renderText("Quit", _font, _c.transform.trans(0 as f64, 512 as f64), &_c.draw_state, _g, self.selection == 1);
        _font.factory.encoder.flush(_device);
    }
}
