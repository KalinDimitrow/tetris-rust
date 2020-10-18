use crate::game_data::*;
use crate::resources::*;
use crate::states::main_menu::MainMenu;
use crate::states::state_machine::*;
use crate::abstraction::piston_abstraction::*;
use crate::abstraction::abstraction_layer::AbstractionLayer;
use piston_window::*;
use std::error;

pub type Abstraction = PistonAbstraction;

pub struct Tetris {
    window: PistonWindow,
    abstraction : Abstraction,
    pub resources: Resources,
    pub data: GameData,
    pub logic: StateMachine,
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

        let mut abstraction  = Abstraction::new()?;
        let resources = Resources::new(resorce_path, &mut window, &mut abstraction)?;
        let game_data = GameData::new()?;
        let game_logic = StateMachine::new(MainMenu::new()?)?;

        Ok(Tetris {
            window,
            abstraction,
            resources: resources,
            data: game_data,
            logic: game_logic,
        })
    }

    pub fn run(&mut self) {
        let abstraction = &mut self.abstraction;
        abstraction.run(&self.logic, &self.resources);
        // while let Some(event) = self.window.next() {
        //     match event {
        //         Event::Loop(_loop) => {
        //             if self.loop_handler(_loop, event) {
        //                 return;
        //             }
        //         }
        //
        //         Event::Input(args, time) => {
        //             self.input_handler(args, time);
        //         }
        //
        //         _ => {}
        //     }
        //
        //     if self.data.running == false {
        //         break;
        //     }
        // }
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
        self.logic.update(&mut self.data, &update, event)
    }

    fn render(&mut self, arguments: RenderArgs, event: Event) {
        let resources = &mut self.resources;
        let game_data = &mut self.data;
        let game_logic = &mut self.logic;
        self.window.draw_2d(&event, |c, g, device| {
            clear([1.0; 4], g);
            // game_logic.render(c, g, &arguments, device, resources, game_data)
        });
    }

    fn input_handler(&mut self, input: Input, time: Option<TimeStamp>) {
        self.logic
            .handle_input(input, time, &mut self.data);
    }
}
