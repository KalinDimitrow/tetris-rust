use crate::game_data::*;
use crate::Resources;
use math::Matrix2d;
use piston_window::*;
use std::error;
use std::vec::Vec;

pub enum StateTransition {
    Push(Box<dyn State>),
    Transition(Box<dyn State>),
    Pop,
    Hold,
}

pub trait State {
    fn update(
        &mut self,
        data: &mut GameData,
        update_args: &UpdateArgs,
        event: Event,
    ) -> StateTransition;

    fn handle_input(&mut self, input: Input, time: Option<TimeStamp>, _data: &mut GameData);

    fn background_render(
        &mut self,
        _c: Context,
        _g: &mut G2d,
        _arguments: &RenderArgs,
        _device: &mut gfx_device_gl::Device,
        _resources: &mut Resources,
        _data: &GameData,
    ) {}

    fn render(
        &mut self,
        _c: Context,
        _g: &mut G2d,
        _arguments: &RenderArgs,
        _device: &mut gfx_device_gl::Device,
        _resources: &mut Resources,
        _data: &GameData,
    ) {}

    fn enter(&mut self, _data: &mut GameData) {}
    fn exit(&mut self, _data: &mut GameData) {}
    fn pause(&mut self, _data: &mut GameData) {}
    fn resume(&mut self, _data: &mut GameData) {}
}

pub struct StateMachine {
    stack: Vec<Box<dyn State>>,
}

impl StateMachine {
    pub fn new(initial_state: Box<dyn State>) -> Result<StateMachine, Box<dyn error::Error>> {
        let stack = vec![initial_state];
        Ok(StateMachine { stack })
    }

    pub fn update(&mut self, data: &mut GameData, update_args: &UpdateArgs, event: Event) -> bool {
        let transition = if let Some(top) = self.stack.last_mut() {
            top.update(data, update_args, event)
        } else {
            StateTransition::Hold
        };

        match transition {
            StateTransition::Push(mut pushed_state) => {
                pushed_state.enter(data);
                self.stack.push(pushed_state);
            }

            StateTransition::Transition(mut transition) => {
                let mut top = self.stack.pop().unwrap();
                top.exit( data);
                transition.enter(data);
                self.stack.push(transition);
            }

            StateTransition::Pop => {
                let stack = &mut self.stack;
                if let Some(mut top) = stack.pop() {
                    top.exit(data);
                }
                if let Some(top) = stack.last_mut() {
                    top.resume(data);
                }
            }

            StateTransition::Hold => {}
        }

        !self.stack.is_empty()
    }

    pub fn handle_input(&mut self, input: Input, time: Option<TimeStamp>, data: &mut GameData) {
        if let Some(top) = self.stack.last_mut() {
            top.handle_input(input, time, data);
        }
    }

    pub fn render(
        &mut self,
        c: Context,
        g: &mut G2d,
        arguments: &RenderArgs,
        device: &mut gfx_device_gl::Device,
        resources: &mut Resources,
        data: &GameData,
    ) {
        self.stack.iter_mut().for_each(|state| {
            state.background_render(c, g, arguments, device, resources, data);
        });
        if let Some(top) = self.stack.last_mut() {
            top.render(c, g, arguments, device, resources, data);
        }
    }
}

pub fn render_text(
    text: &str,
    font: &mut Glyphs,
    transform: Matrix2d,
    draw_state: &DrawState,
    g: &mut G2d,
    selected: bool,
) {
    let color_pair: (f32, f32) = if selected { (0.2, 1.0) } else { (1.0, 0.2) };

    text::Text::new_color([color_pair.0, color_pair.1, 0.0, 1.0], 128)
        .draw(text, font, draw_state, transform, g)
        .unwrap();
}
