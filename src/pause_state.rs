use crate::game_data::*;
use crate::paly_state::PlayState;
use crate::state_machine::*;
use crate::GameResources;
use piston_window::*;
use std::error;

const ELEMENTS_COUNT: i32 = 2;

pub struct Pause {
    interact: bool,
}

impl Pause {
    pub fn new() -> Result<Box<dyn State>, Box<dyn error::Error>> {
        Ok(Box::new(Pause {
            interact: false,
        }))
    }
}

impl State for Pause {
    fn update(
        &mut self,
        _data: &mut GameData,
        _update_args: &UpdateArgs,
        _event: Event,
    ) -> StateTransition {
        if self.interact {
            self.interact = false;
            return StateTransition::Pop;
        }
        StateTransition::Hold
    }

    fn handle_input(&mut self, input: Input, _time: Option<TimeStamp>, _data: &mut GameData) {
        match input {
            Input::Button(buttons) => {
                if buttons.state == ButtonState::Press {
                    match buttons.button {
                        Button::Keyboard(key) => match key {
                            Key::Escape => {
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
        c: Context,
        g: &mut G2d,
        arguments: &RenderArgs,
        device: &mut gfx_device_gl::Device,
        resources: &mut GameResources,
        data: &GameData,
    ) {
        let texture = &resources.background;
        let font = &mut resources.font;

        clear([1.0; 4], g);
        image(texture, c.transform, g);
        text::Text::new_color([0.2, 0.8, 0.3, 1.0], 32)
            .draw(
                "Pause",
                font,
                &c.draw_state,
                c.transform
                    .trans(0 as f64, 128 as f64),
                g,
            )
            .unwrap();

        font.factory.encoder.flush(device);
    }
}
