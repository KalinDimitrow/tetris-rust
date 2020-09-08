use crate::game_data::{GameField, HEIGHT, WIDTH};
use crate::tetramino::Point;
use crate::tetramino::TetrominoType;

fn valid_block(block: &TetrominoType) -> bool {
    match block {
        TetrominoType::E => {
            return false;
        }
        _ => {
            return true;
        }
    }
}

fn find_neighbours(visited: &mut Vec<bool>, game_field: &GameField, index: usize, offset : usize) -> Vec<usize> {
    let row = index / WIDTH;
    let collumn = index % WIDTH;
    let mut result: Vec<usize> = Vec::new();

    if collumn > 0 {
        let new_index = index - 1;
        if !visited[new_index] {
            visited[new_index] = true;
            if valid_block(&game_field[new_index]) {
                result.push(new_index);
            }
        }
    }

    if collumn < WIDTH - 1 {
        let new_index = index + 1;
        if !visited[new_index] {
            visited[new_index] = true;
            if valid_block(&game_field[new_index]) {
                result.push(new_index);
            }
        }
    }

    if row > 0 {
        let new_index = index - WIDTH;
        if !visited[new_index] {
            visited[new_index] = true;
            if valid_block(&game_field[new_index]) {
                result.push(new_index);
            }
        }
    }

    if row < HEIGHT - offset - 1 {
        let new_index = index + WIDTH;
        if !visited[new_index] {
            visited[new_index] = true;
            if valid_block(&game_field[new_index]) {
                result.push(new_index);
            }
        }
    }

    result
}

fn flood_field(game_field: &mut GameField, begin: usize) -> Vec<Vec<Point>> {
    let mut result: Vec<Vec<Point>> = Vec::new();
    let size = (HEIGHT - begin) * WIDTH;
    let mut visited: Vec<bool> = vec![false; size];
    for index in 0..size {
        if visited[index] {
            continue;
        }

        match game_field[index] {
            TetrominoType::E => {
                visited[index] = true;
                continue;
            }
            _ => {
                visited[index] = true;
                let mut stack: Vec<usize> = vec![index];
                let mut chunk_array: Vec<Point> = Vec::new();

                while !stack.is_empty() {
                    let current = stack.pop().unwrap();

                    chunk_array.push(Point {
                        x: (current % WIDTH) as i32,
                        y: (current / WIDTH) as i32,
                    });

                    game_field[current] = TetrominoType::E;

                    stack.append(&mut find_neighbours(&mut visited, game_field, current, begin));
                }

                if !chunk_array.is_empty() {
                    result.push(chunk_array);
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_flood_field_1() {
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
        let result = flood_field(&mut gamefield, 0);
        assert_eq!(0, result.len());
    }

    #[test]
    fn test_flood_field_2() {
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
            E, I, I, E, E, I, I, I, E, E,
            E, I, I, E, E, I, I, I, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, I, I, E, E, E, E, E,
            E, E, E, I, I, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
        ];
        let result = flood_field(&mut gamefield, 0);
        assert_eq!(3, result.len());
    }

    #[test]
    fn test_flood_field_3() {
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
            E, I, I, E, E, I, I, I, E, E,
            E, I, I, E, E, I, I, I, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, I, I, E, E, E, E, E,
            E, E, E, I, I, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
        ];
        let result = flood_field(&mut gamefield, 7);

        let mut count = 0;
        for chunk in &result {
            print!("Chunck {}\n", count);
            for point in chunk {
                print!("[{},{}] ",point.x, point.y);
            }
            count += 1;
            print!("\n");
        }
        assert_eq!(2, result.len());
    }

    #[test]
    fn test_flood_field_4() {
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
            E, E, E, E, I, E, E, E, E, E,
            E, E, E, E, I, I, E, E, E, E,
            I, I, I, I, I, I, E, E, I, E,
            E, E, E, E, E, E, E, E, E, E,
        ];
        let result = flood_field(&mut gamefield, 1);

        let mut count = 0;
        for chunk in &result {
            print!("Chunck {}\n", count);
            for point in chunk {
                print!("[{},{}] ",point.x, point.y);
            }
            count += 1;
            print!("\n");
        }
        assert_eq!(2, result.len());
    }
}

pub fn find_chunks(game_field: &mut GameField, begin: usize) -> Vec<Chunk> {
    let chunks_data = flood_field(game_field, begin);
    let mut result = Vec::new();
    for data in chunks_data {
        result.push(Chunk::new(data));
    }
    result
}

pub struct Chunk {
    pub position: Point,
    pub elements: Vec<Point>,
}

impl Chunk {
    pub fn new(data: Vec<Point>) -> Self {
        Chunk {
            position: Point { x: 0, y: 0 },
            elements: data,
        }
    }

    pub fn iterator(&self) -> std::slice::Iter<Point> {
        self.elements.iter()
    }
}

impl<'a> IntoIterator for &'a Chunk {
    type Item = Point;
    type IntoIter = ChunkIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            index: 0,
            payload: self,
        }
    }
}

pub struct ChunkIterator<'a> {
    index: usize,
    payload: &'a Chunk,
}

impl<'a> Iterator for ChunkIterator<'a> {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.payload.elements.len() {
            let result = self.payload.elements[self.index];
            self.index += 1;
            return Some(result);
        }

        None
    }
}
