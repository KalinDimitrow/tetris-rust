use crate::figure_fall_state::FallingState;
use crate::game_data::*;
use crate::state_machine::*;
use crate::tetramino::*;
use crate::GameResources;
use piston_window::*;
use std::error;

pub struct PlayState {
    logic: StateMachine,
}

pub fn check_for_collision(
    position: &Point,
    sequence: &TetrominoRotation,
    game_field: &[PlayBlock; WIDTH * HEIGHT],
) -> bool {
    for element in sequence.into_iter() {
        let new_position = position.add(&element);
        let index = new_position.x + (WIDTH as i32) * new_position.y;
        if new_position.y < 0 {
            continue;
        }

        if new_position.x < 0 || new_position.x >= WIDTH as i32 {
            return true;
        }

        if new_position.y >= HEIGHT as i32 {
            return true;
        }

        match game_field[index as usize] {
            PlayBlock::E => {}

            _ => {
                return true;
            }
        }
    }

    false
}

fn draw_play_field(
    c: &Context,
    g: &mut G2d,
    _arguments: &RenderArgs,
    _device: &mut gfx_device_gl::Device,
    resources: &mut GameResources,
    data: &GameData,
) {
    let empty_block = &resources.empty_block;
    let full_block = &resources.cube_block;
    let blocks = &data.play_table;
    let mut position_index: usize = 0;
    blocks.iter().for_each(|block: &PlayBlock| {
        let x = position_index % GAME_FIELD_WIDTH;
        let y = (position_index / GAME_FIELD_WIDTH) * BLOCK_SIZE;
        position_index += BLOCK_SIZE;
        match block {
            PlayBlock::E => {
                image(empty_block, c.transform.trans(x as f64, y as f64), g);
            }

            _ => {
                image(full_block, c.transform.trans(x as f64, y as f64), g);
            }
        }
    });
}

fn draw_score(
    c: &Context,
    g: &mut G2d,
    _arguments: &RenderArgs,
    device: &mut gfx_device_gl::Device,
    resources: &mut GameResources,
    data: &GameData,
) {
    let score = data.score;

    text::Text::new_color([1.0, 1.0, 0.0, 1.0], 32)
        .draw(
            "Score : ",
            &mut resources.font,
            &c.draw_state,
            c.transform
                .trans(SCORE_TEXT_POSITION_X as f64, SCORE_TEXT_POSITION_Y as f64),
            g,
        )
        .unwrap();

    text::Text::new_color([1.0, 1.0, 0.0, 1.0], 16)
        .draw(
            &score.to_string(),
            &mut resources.font,
            &c.draw_state,
            c.transform
                .trans(SCORE_POSITION_X as f64, SCORE_POSITION_Y as f64),
            g,
        )
        .unwrap();

    resources.font.factory.encoder.flush(device);
}

fn draw_preview(
    c: &Context,
    g: &mut G2d,
    _arguments: &RenderArgs,
    _device: &mut gfx_device_gl::Device,
    resources: &mut GameResources,
    data: &GameData,
) {
    let full_block = &resources.cube_block;
    let sequence = data.tetramino_preview_sequence();
    let offset = data.tetramino_preview_offset();
    sequence.iter().for_each(|position: &Point| {
        let x =
            PREVIEW_DEFAULT_POSITION_X + (position.x * BLOCK_SIZE as i32) as f64 + offset.x as f64;
        let y =
            PREVIEW_DEFAULT_POSITION_Y + (position.y * BLOCK_SIZE as i32) as f64 + offset.y as f64;
        image(full_block, c.transform.trans(x as f64, y as f64), g);
    });
}

fn draw_current(
    c: &Context,
    g: &mut G2d,
    _arguments: &RenderArgs,
    _device: &mut gfx_device_gl::Device,
    resources: &mut GameResources,
    data: &GameData,
) {
    let current = &data.current_figure;
    let type_index = current.get_type();
    let rotation_index = current.get_rotation();
    let position = current.get_position();
    let rotation = &data.tetraminoes_data[type_index].rotations[rotation_index];
    let full_block = &resources.cube_block;

    rotation.into_iter().for_each(|offset: Point| {
        let x = (position.x * BLOCK_SIZE as i32) as f64 + (offset.x * BLOCK_SIZE as i32) as f64;
        let y = (position.y * BLOCK_SIZE as i32) as f64 + (offset.y * BLOCK_SIZE as i32) as f64;
        image(full_block, c.transform.trans(x as f64, y as f64), g);
    });
}

impl PlayState {
    pub fn new() -> Result<Box<dyn State>, Box<dyn error::Error>> {
        Ok(Box::new(PlayState {
            logic: StateMachine::new(FallingState::new()?)?,
        }))
    }
}

impl State for PlayState {
    fn update(
        &mut self,
        data: &mut GameData,
        update_args: &UpdateArgs,
        event: Event,
    ) -> StateTransition {
        if self.logic.update(data, update_args, event) {
            StateTransition::Hold
        } else {
            StateTransition::Pop
        }
    }

    fn handle_input(&mut self, input: Input, time: Option<TimeStamp>, data: &mut GameData) {
        self.logic.handle_input(input, time, data);
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
        clear([1.0; 4], g);
        let background = &resources.background;
        image(background, c.transform, g);
        draw_play_field(&c, g, arguments, device, resources, data);
        draw_score(&c, g, arguments, device, resources, data);
        draw_preview(&c, g, arguments, device, resources, data);
        draw_current(&c, g, arguments, device, resources, data);
        self.logic.render(c, g, arguments, device, resources, data);
    }
}
