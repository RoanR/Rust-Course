use std::path::PathBuf;

use blackjack::Game;
use clap::Parser;
use cli::Args;

mod blackjack;
mod cards;
mod cli;
mod terminal;

fn init_game(load: bool, path: PathBuf) -> Game {
    if load {
        match Game::load(path.clone()) {
            Ok(g) => return g,
            Err(_) => return Game::new(path),
        };
    };
    Game::new(path)
}

fn main() {
    let args = Args::parse();
    let mut game = init_game(args.load_game(), args.path_game().to_path_buf());

    println!("{}", game);
    game.turn();
    game.cpu_turn();
    if game.human_win() {
        println!("THE HUMAN WON!");
    } else {
        println!("the human lost");
    }
}

#[cfg(test)]
mod tests {
    use crate::{blackjack::Game, init_game};
    use std::path::Path;
    use tempfile::NamedTempFile;

    #[test]
    fn init_game_test() {
        // load being false should give a new game
        let mut game = init_game(false, Path::new("test").to_path_buf());
        // load being true but path being fake should give new game
        game = init_game(true, Path::new("test").to_path_buf());
        // load being true and path being real should load the game
        let f = NamedTempFile::new().unwrap();
        let save_game = Game::new(f.path().to_path_buf());
        assert!(save_game.save().is_ok());
        let load_game = init_game(true, f.path().to_path_buf());
        assert_eq!(save_game, load_game)
    }
}
