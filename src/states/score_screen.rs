use crate::states::state_machine::*;
use crate::states::main_menu::*;
use crate::game_data::*;
use crate::Resources;
use piston_window::*;
use std::error;

pub struct ScoreScreen {
    interact: bool,
    score : u32,
    level : u32,
}

impl ScoreScreen {
    pub fn new(score : u32, level : u32) -> Result<Box<dyn State>, Box<dyn error::Error>> {
        Ok(Box::new(ScoreScreen {
            interact: false,
            score,
            level
        }))
    }
}

impl State for ScoreScreen {
    fn update(
        &mut self,
        _data: &mut GameData,
        _update_args: &UpdateArgs,
        _event: Event,
    ) -> StateTransition {
        if self.interact {
            self.interact = false;
            return StateTransition::Transition(MainMenu::new().unwrap());
        }
        StateTransition::Hold
    }

    fn handle_input(&mut self, input: Input, _time: Option<TimeStamp>, _data: &mut GameData) {
        match input {
            Input::Button(buttons) => {
                if buttons.state == ButtonState::Press {
                    match buttons.button {
                        Button::Keyboard(key) => match key {
                            Key::Return => {
                                self.interact = true;
                            }

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
        _arguments: &RenderArgs,
        _device: &mut gfx_device_gl::Device,
        resources: &mut Resources,
        _data: &GameData,
    ) {
        let texture = &resources.background;
        let font = &mut resources.font;

        clear([1.0; 4], g);
        image(texture, c.transform, g);

        text::Text::new_color([0.2, 0.8, 0.3, 1.0], 32)
            .draw(
                "Score : ",
                font,
                &c.draw_state,
                c.transform
                    .trans(0 as f64, 128 as f64),
                g,
            )
            .unwrap();

        text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32)
            .draw(
                &self.score.to_string(),
                font,
                &c.draw_state,
                c.transform
                    .trans(128 as f64, 128 as f64),
                g,
            )
            .unwrap();

        text::Text::new_color([0.2, 0.8, 0.3, 1.0], 32)
            .draw(
                "Level : ",
                font,
                &c.draw_state,
                c.transform
                    .trans(0 as f64, 228 as f64),
                g,
            )
            .unwrap();

        text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32)
            .draw(
                &self.level.to_string(),
                font,
                &c.draw_state,
                c.transform
                    .trans(128 as f64, 228 as f64),
                g,
            )
            .unwrap();

        text::Text::new_color([0.2, 0.8, 0.3, 1.0], 32)
            .draw(
                "Press Enter to return to main menu",
                font,
                &c.draw_state,
                c.transform
                    .trans(0 as f64, 328 as f64),
                g,
            )
            .unwrap();

        font.factory.encoder.flush(_device);
    }
}
