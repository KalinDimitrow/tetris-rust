extern crate gfx_device_gl;
use crate::state_machine::*;
use crate::main_menu_state::*;
use crate::paly_state::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::error;
use std::thread::current;

pub struct GameLogic {
    pub states : std::collections::HashMap<&'static str, Rc<RefCell<dyn State>>>,
    pub current_state : Rc<RefCell<dyn State>>,
}

impl GameLogic {
    pub fn new() -> Result<Rc<RefCell<GameLogic>>,Box<dyn error::Error>> {
        let default_state = MainMenu::new()?;
        let mut states: HashMap<&'static str, Rc<RefCell<dyn State>>> = HashMap::new();
        states.insert(crate::state_machine::MAIN_MENU, default_state.clone());
        let play_state = PlayState::new()?;
        states.insert(crate::state_machine::PLAY_STATE, play_state);
        Ok(Rc::new(RefCell::new(GameLogic{states, current_state : default_state })))
    }

    pub fn transition(&mut self, next : &'static str) {

        self.current_state = self.states.get(next).unwrap().clone();
    }
}