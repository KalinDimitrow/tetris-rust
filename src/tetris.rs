use crate::game_data::*;
use crate::game_resources::*;
use crate::main_menu_state::MainMenu;
use crate::state_machine::*;
use piston_window::*;
use std::error;

pub struct Tetris {
    window: PistonWindow,
    pub game_resources: GameResources,
    pub game_data: GameData,
    pub game_logic: StateMachine,
}

const GAME_NAME: &str = "Tetris";
const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 1280;

impl Tetris {
    pub fn new(resorce_path: &str) -> Result<Tetris, Box<dyn error::Error>> {
        let opengl = OpenGL::V3_2;
        let mut window: PistonWindow =
            WindowSettings::new(GAME_NAME, [WINDOW_WIDTH, WINDOW_HEIGHT])
                .exit_on_esc(false)
                .graphics_api(opengl)
                .build()?;

        let game_resources = GameResources::new(resorce_path, &mut window)?;
        let game_data = GameData::new()?;
        let game_logic = StateMachine::new(MainMenu::new()?)?;

        Ok(Tetris {
            window,
            game_resources,
            game_data,
            game_logic,
        })
    }

    pub fn run(&mut self) {
        while let Some(event) = self.window.next() {
            match event {
                Event::Loop(_loop) => {
                    if self.loop_handler(_loop, event) {
                        return;
                    }
                }

                Event::Input(args, time) => {
                    self.input_handler(args, time);
                }

                _ => {}
            }

            if self.game_data.running == false {
                break;
            }
        }
    }

    fn loop_handler(&mut self, loop_arg: Loop, event: Event) -> bool {
        match loop_arg {
            Loop::Update(update_args) => {
                if !self.update(update_args, event.clone()) {
                    return true;
                }
            }

            Loop::Render(render_args) => {
                self.render(render_args, event);
            }

            _ => {}
        }

        false
    }

    fn update(&mut self, update: UpdateArgs, event: Event) -> bool {
        self.game_logic.update(&mut self.game_data, &update, event)
    }

    fn render(&mut self, _arguments: RenderArgs, event: Event) {
        let resources = &mut self.game_resources;
        let game_data = &mut self.game_data;
        let game_logic = &mut self.game_logic;
        self.window.draw_2d(&event, |c, g, device| {
            clear([1.0; 4], g);
            game_logic.render(c, g, &_arguments, device, resources, game_data)
        });
    }

    fn input_handler(&mut self, input: Input, time: Option<TimeStamp>) {
        self.game_logic
            .handle_input(input, time, &mut self.game_data);
    }
}
