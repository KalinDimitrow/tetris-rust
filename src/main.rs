use libtetrisgame::Tetris;

fn main() {
    let result = Tetris::new("");
    match result {
        Ok(mut game) => {
            game.run();
        },
        Err(error) => {
            println!("{}", error);
        }
    }

}
