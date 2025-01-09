use std::path::PathBuf;

use blackjack::Game;

mod blackjack;
mod cards;
mod cli;
mod terminal;

fn init_game(load: bool, path: PathBuf) -> Game {
    if load {
        match Game::load(path) {
            Ok(g) => return g,
            Err(_) => return Game::new(),
        };
    };
    Game::new()
}
fn main() {
    let mut game = Game::new();
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
        let save_game = Game::new();
        let f = NamedTempFile::new().unwrap();
        assert!(save_game.save(f.path().to_path_buf()).is_ok());
        let load_game = init_game(true, f.path().to_path_buf());
        assert_eq!(save_game, load_game)
    }
}
