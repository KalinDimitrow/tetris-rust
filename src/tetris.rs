use crate::game_resources::*;
use crate::game_data::*;
use crate::game_logic::*;
use piston_window::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::error;

pub struct Tetris {
    window : PistonWindow,
    pub game_resources : GameResources,
    // pub game_data : Rc<RefCell<GameData>>,
    pub game_data : GameData,
    pub game_logic : Rc<RefCell<GameLogic>>,
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

        let _game_resources = GameResources::new(resorce_path, &mut window)?;
        let _game_data = GameData::new()?;
        let _game_logic = GameLogic::new()?;

        Ok(Tetris{
            window : window,
            game_resources : _game_resources,
            game_data : _game_data,
            game_logic : _game_logic
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
        let next = self.game_logic.borrow_mut().current_state.borrow_mut()
            .update(&mut self.game_data, &update, event);
        self.game_logic.borrow_mut().transition(next);
    }

    fn render(&mut self, _arguments: RenderArgs, event : Event) {
        let resources = &mut self.game_resources;
        let current_state = self.game_logic.borrow().current_state.clone();
        let game_data = &mut self.game_data;
        self.window.draw_2d(&event, |c, g, _device| {
            clear([1.0; 4], g);
            current_state.borrow_mut().render( c, g, &_arguments, _device, resources, game_data);
        });
    }

    fn inputHandler(&mut self, input : Input, time : Option<TimeStamp>) {
        let next = self.game_logic.borrow_mut().current_state.borrow_mut()
            .handleInput(input, time, &mut self.game_data);
        self.game_logic.borrow_mut().transition(next);
    }
}