use crate::states::line_clearing::*;
use crate::states::state_machine::*;
use crate::states::play::*;
use crate::game_data::*;
use crate::tetramino::*;
use crate::Resources;
use crate::chunk::*;
use piston_window::*;
use std::error;

const TIME_INTERVAL: f64 = 0.03;

pub struct ChunkFall {
    chunks : Vec<Chunk>,
    iteration : i32,
    fall_time: f64,
    begin : usize,
}

impl ChunkFall {
    pub fn new(begin : usize) -> Result<Box<dyn State>, Box<dyn error::Error>> {
        Ok(Box::new(ChunkFall {
            chunks : vec![],
            iteration : 0,
            fall_time : 0.0,
            begin,
        }))
    }
}

impl State for ChunkFall {
    fn update(
        &mut self,
        data: &mut GameData,
        update_args: &UpdateArgs,
        _event: Event,
    ) -> StateTransition {
        if self.chunks.is_empty() {
            return StateTransition::Transition(LineClearing::new().unwrap());
        }

        self.fall_time += update_args.dt;
        let time_interval = TIME_INTERVAL / data.speed_multiplier();
        if self.fall_time >= time_interval {
            self.fall_time -= time_interval;

            let play_table = &mut data.play_table;
            let iteration = self.iteration;
            self.chunks.retain(|chunk : &Chunk| {
                let position = Point{x : chunk.position.x, y : chunk.position.y + iteration + 1};
                if check_for_collision(&position, chunk.into_iter(), play_table) {
                    let position = Point{x : chunk.position.x, y : chunk.position.y + iteration};
                    fill_field(&position, chunk.into_iter(), play_table);
                    return false;
                }
                true
            });
            self.iteration += 1;
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
        resources: &mut Resources,
        _data: &GameData,
    ) {
        let full_block = &resources.cube_block;
        let iteration = self.iteration;

        self.chunks.iter().for_each(|chunk : &Chunk| {
            let position = Point{x : chunk.position.x, y : chunk.position.y + iteration};
            chunk.elements.iter().for_each(|offset: &Point| {
                let x = (position.x * BLOCK_SIZE as i32) as f64 + (offset.x * BLOCK_SIZE as i32) as f64;
                let y = (position.y * BLOCK_SIZE as i32) as f64 + (offset.y * BLOCK_SIZE as i32) as f64;
                image(full_block, c.transform.trans(x as f64, y as f64), g);
            });
        });
    }

    fn enter(&mut self, data: &mut GameData) {
        let play_table = &mut data.play_table;
        self.chunks = find_chunks(play_table, HEIGHT - self.begin);
    }
}
