use crate::game_resources::*;
use crate::state_machine::*;
use crate::game_data::*;
use piston_window::*;
use std::error;
use crate::main_menu_state::MainMenu;

pub struct Tetris {
    window : PistonWindow,
    pub game_resources : GameResources,
    pub game_data : GameData,
    pub game_logic : StateMachine,
}

const GAME_NAME : &str = "Tetris";
const WINDOW_WIDTH : u32 = 1024;
const WINDOW_HEIGHT : u32 = 1280;

impl Tetris {
    pub fn new(resorce_path : &str) -> Result<Tetris, Box<dyn error::Error>> {
        let opengl = OpenGL::V3_2;
        let mut window: PistonWindow =
            WindowSettings::new(GAME_NAME, [WINDOW_WIDTH, WINDOW_HEIGHT])
            .exit_on_esc(false)
            .graphics_api(opengl)
            .build()?;

        let game_resources = GameResources::new(resorce_path, &mut window)?;
        let game_data = GameData::new()?;
        let game_logic = StateMachine::new(MainMenu::new()?)?;

        Ok(Tetris{
            window,
            game_resources,
            game_data,
            game_logic
        })
    }

    pub fn run (&mut self) {
        while let Some(event) = self.window.next() {
            match event {
                Event::Loop(_loop) => {
                    self.loop_handler(_loop, event);
                }

                Event::Input(args,time) => {
                    self.inputHandler(args, time);
                }

                _ => {

                }
            }

            if self.game_data.running == false {
                break;
            }
        }
    }

    fn loop_handler(&mut self, loop_arg : Loop, event : Event) {
        match loop_arg {

            Loop::Update(update_args) => {
                self.update(update_args, event.clone());
            }

            Loop::Render(render_args) => {
                self.render(render_args, event);
            }

            _ => {

            }
        }
    }

    fn update(&mut self, update : UpdateArgs, event : Event) {
        self.game_logic.update(&mut self.game_data, &update, event);
    }

    fn render(&mut self, _arguments: RenderArgs, event : Event) {
        let resources = &mut self.game_resources;
        let game_data = &mut self.game_data;
        let game_logic = &mut self.game_logic;
        self.window.draw_2d(&event, |c, g, device| {
            clear([1.0; 4], g);
            game_logic.render( c, g, &_arguments, device, resources, game_data)
        });
    }

    fn inputHandler(&mut self, input : Input, time : Option<TimeStamp>) {
        self.game_logic.handleInput(input, time, &mut self.game_data);
    }
}