use crate::states::state_machine::*;
use crate::states::score_screen::*;
use crate::states::pause::*;
use crate::states::fall::*;
use crate::game_data::*;
use crate::tetramino::*;
use crate::Resources;
use crate::chunk::*;
use piston_window::*;
use std::error;

pub const BLOCK_SIZE: usize = 64;
pub const GAME_FIELD_WIDTH: usize = BLOCK_SIZE * crate::game_data::WIDTH;
const SCORE_TEXT_SIZE: u32 = 32;
const SCORE_POSITION_X: f64 = 780.0;
const SCORE_POSITION_Y: f64 = 24.0;
const SCORE_TEXT_POSITION_X: f64 = 650.0;
const SCORE_TEXT_POSITION_Y: f64 = SCORE_TEXT_SIZE as f64;
const LEVEL_POSITION_X: f64 = 780.0;
const LEVEL_POSITION_Y: f64 = 74.0;
const LEVEL_TEXT_POSITION_X: f64 = 650.0;
const LEVEL_TEXT_POSITION_Y: f64 = SCORE_TEXT_SIZE as f64 + 50.0;
const PREVIEW_DEFAULT_POSITION_X: f64 = 780.0;
const PREVIEW_DEFAULT_POSITION_Y: f64 = 240.0;

pub struct PlayState {
    logic: StateMachine,
    pause_event : bool,
}

pub fn land_flying_chunks(play_table: &mut GameField, begin : usize) {
    let mut chunks = find_chunks(play_table, HEIGHT - begin);

    let mut iteration : i32 = 0;
    while !chunks.is_empty() {

        chunks.retain(|chunk : &Chunk| {
            let position = Point{x : chunk.position.x, y : chunk.position.y + iteration + 1};
            if check_for_collision(&position, chunk.into_iter(), play_table) {
                let position = Point{x : chunk.position.x, y : chunk.position.y + iteration};
                fill_field(&position, chunk.into_iter(), play_table);
                return false;
            }
         true
        });
        iteration += 1;
    }

}

pub fn find_filled_lines(play_table: &GameField) -> Vec<usize> {
    let mut lines: Vec<usize> = Vec::new();
    for row in 0..HEIGHT {
        let mut line = true;
        for column in 0..WIDTH {
            match play_table[row * WIDTH + column] {
                TetrominoType::E => {
                    line = false;
                    break;
                }
                _ => {}
            }
        }

        if line {
            lines.push(row);
        }
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_filled_lines_1() {
        const E: TetrominoType = TetrominoType::E;
        let mut gamefield: GameField = [
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
        ];
        let result = find_filled_lines(&mut gamefield);
        assert_eq!(0, result.len());
    }

    #[test]
    fn test_find_filled_lines_2() {
        const E: TetrominoType = TetrominoType::E;
        const I: TetrominoType = TetrominoType::I;
        let mut gamefield: GameField = [
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            I, I, I, I, I, I, I, I, I, I,
            I, E, I, I, I, I, I, I, I, I,
            I, I, I, I, I, I, I, I, I, I,
            I, I, I, I, I, I, I, I, I, I,
        ];
        let result = find_filled_lines(&mut gamefield);
        assert_eq!(3, result.len());
    }
}

pub fn clear_play_table(play_table: &mut GameField, lines: Vec<usize>) {
    for line in lines {
        for element in 0..WIDTH {
            play_table[line * WIDTH + element] = TetrominoType::E;
        }
    }
}

#[allow(dead_code)]
pub fn score(data: &mut GameData) {
    let mut lines_count = 0;
    let score_multiplier = data.score_multiplier();
    loop {
        let play_table = &data.play_table;
        let lines = find_filled_lines(play_table);
        let count = lines.len();
        if count != 0 {
            lines_count += count;
            let play_table = &mut data.play_table;
            let chunk_begin : usize = lines.last().unwrap().clone();
            clear_play_table(play_table, lines);
            land_flying_chunks(play_table, chunk_begin);
            data.add_score((1 << (count - 1)) * score_multiplier);
        } else {
            break;
        }
    }

    data.add_score((lines_count*(lines_count + 1)) as u32 * score_multiplier);
}

pub fn fill_field(
    position: &Point,
    sequence: impl IntoIterator<Item = Point>,
    game_field: &mut GameField,
) {
    for element in sequence {
        let element_position = position.add(&element);
        if element_position.y >= 0 {
            let index = element_position.x as usize + (element_position.y as usize) * WIDTH;
            game_field[index] = TetrominoType::O;
        }
    }
}

pub fn check_for_collision(
    position: &Point,
    sequence: impl IntoIterator<Item = Point>,
    game_field: &GameField,
) -> bool {
    for element in sequence {
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
            TetrominoType::E => {}

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
    resources: &mut Resources,
    data: &GameData,
) {
    let empty_block = &resources.empty_block;
    let full_block = &resources.cube_block;
    let blocks = &data.play_table;
    let mut position_index: usize = 0;
    blocks.iter().for_each(|block: &TetrominoType| {
        let x = position_index % GAME_FIELD_WIDTH;
        let y = (position_index / GAME_FIELD_WIDTH) * BLOCK_SIZE;
        position_index += BLOCK_SIZE;
        match block {
            TetrominoType::E => {
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
    resources: &mut Resources,
    data: &GameData,
) {
    let score = data.score;
    let level = data.dificulty;

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

    text::Text::new_color([1.0, 1.0, 0.0, 1.0], 32)
        .draw(
            "Level : ",
            &mut resources.font,
            &c.draw_state,
            c.transform
                .trans(LEVEL_TEXT_POSITION_X as f64, LEVEL_TEXT_POSITION_Y as f64),
            g,
        )
        .unwrap();

    text::Text::new_color([1.0, 1.0, 0.0, 1.0], 16)
        .draw(
            &level.to_string(),
            &mut resources.font,
            &c.draw_state,
            c.transform
                .trans(LEVEL_POSITION_X as f64, LEVEL_POSITION_Y as f64),
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
    resources: &mut Resources,
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

pub fn draw_current(
    c: &Context,
    g: &mut G2d,
    _arguments: &RenderArgs,
    _device: &mut gfx_device_gl::Device,
    resources: &mut Resources,
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
            pause_event : false,
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
        if self.pause_event {
            self.pause_event = false;
            StateTransition::Push(Pause::new().unwrap())
        } else if self.logic.update(data, update_args, event) {
            StateTransition::Hold
        } else {
            StateTransition::Transition(ScoreScreen::new(data.score, data.dificulty).unwrap())
        }
    }

    fn handle_input(&mut self, input: Input, time: Option<TimeStamp>, data: &mut GameData) {
        match input {
            Input::Button(buttons) => match buttons.button {
                Button::Keyboard(key) => match key {
                    Key::Escape => {
                        if buttons.state == ButtonState::Press {
                            self.pause_event = true;
                        }
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }

        self.logic.handle_input(input, time, data);
    }

    fn render(
        &mut self,
        c: Context,
        g: &mut G2d,
        arguments: &RenderArgs,
        device: &mut gfx_device_gl::Device,
        resources: &mut Resources,
        data: &GameData,
    ) {
        clear([1.0; 4], g);
        let background = &resources.background;
        image(background, c.transform, g);
        draw_play_field(&c, g, arguments, device, resources, data);
        draw_score(&c, g, arguments, device, resources, data);
        draw_preview(&c, g, arguments, device, resources, data);
        // self.logic.render(c, g, arguments, device, resources, data);
    }

    fn enter(&mut self, data: &mut GameData) {
        data.dificulty = 0;
        data.score = 0;
        data.play_table = [TetrominoType::E; WIDTH * HEIGHT];
        data.current_figure = Tetramino::new(GameData::random_tetramino_index());
    }
}
