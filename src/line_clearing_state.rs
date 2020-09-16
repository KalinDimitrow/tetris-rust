use crate::game_data::*;
use crate::paly_state::*;
use crate::state_machine::*;
use crate::GameResources;
use crate::tetramino::*;
use crate::chunk_falling_state::*;
use piston_window::*;
use std::error;

const TIME_INTERVAL : f64 = 0.1;
const MAX_ITERATIONS : u32 = 5;

pub struct LineClearing {
    min_line : usize,
    max_line : usize,
    line_count : usize,
    iterations : u32,
    time_passed : f64,
    lines : Vec<usize>,
}

impl LineClearing {
    pub fn new() -> Result<Box<dyn State>, Box<dyn error::Error>> {
        Ok(Box::new(LineClearing {
            min_line : 0,
            max_line : 0,
            line_count : 0,
            iterations : 0,
            time_passed : 0.0,
            lines : vec![],
        }))
    }
}

impl State for LineClearing {
    fn update(
        &mut self,
        data: &mut GameData,
        update_args: &UpdateArgs,
        _event: Event,
    ) -> StateTransition {
        if self.line_count == 0 {
            let overall_lines = data.lines;
            data.score += (((overall_lines + 1) * overall_lines) * 50 )as u32;
            data.lines = 0;
            return StateTransition::Pop;
        }

        self.time_passed += update_args.dt;
        if self.time_passed >= TIME_INTERVAL {
            self.time_passed -= TIME_INTERVAL;
            self.iterations += 1;
            if self.iterations >= MAX_ITERATIONS {
                return StateTransition::Transition(ChunkFall::new(self.min_line).unwrap());
            }
        }
        StateTransition::Hold
    }

    fn handle_input(&mut self, _input: Input, _time: Option<TimeStamp>, _data: &mut GameData) {
    }

    fn render(
        &mut self,
        c: Context,
        g: &mut G2d,
        _arguments: &RenderArgs,
        _device: &mut gfx_device_gl::Device,
        resources: &mut GameResources,
        data: &GameData,
    ) {
        let empty_block = &resources.empty_block;
        let a = self.max_line*WIDTH;
        let b = (self.min_line + 1)*WIDTH;
        let blocks = &data.play_table[a..b];
        let mut position_index: usize = a*BLOCK_SIZE;
        blocks.iter().for_each(|_block: &TetrominoType| {
            let x = position_index % GAME_FIELD_WIDTH;
            let y = (position_index / GAME_FIELD_WIDTH) * BLOCK_SIZE;
            position_index += BLOCK_SIZE;
            if self.iterations%2 == 0 {
                image(empty_block, c.transform.trans(x as f64, y as f64), g);
            }

        });
    }

    fn enter(&mut self, _state_machine: &mut StateMachine, data: &mut GameData) {
        let play_table = &data.play_table;
        self.lines = find_filled_lines(play_table);
        data.lines += self.line_count;
        let count = self.lines.len();
        if count != 0 {
            self.max_line = self.lines.first().unwrap().clone();
            self.min_line = self.lines.last().unwrap().clone();
            self.line_count = count;
        }
    }

    fn exit(&mut self, _state_machine: &mut StateMachine, data: &mut GameData) {
        let lines_count = self.line_count;
        if lines_count != 0 {
            let play_table = &mut data.play_table;
            clear_play_table(play_table, self.lines.clone());
            data.score += (1 << (self.line_count - 1)) * 100;
        }

        data.score += (lines_count*(lines_count + 1)) as u32 * 50;
        data.current_figure = Tetramino::new(data.next_figure);
        data.next_figure = GameData::random_tetramino_index();
    }
}
