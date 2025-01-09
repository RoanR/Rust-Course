use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about = "Blackjack Game", name = "bjg")]
pub struct Args {
    /// The path to save to/load the game to/from
    #[arg(name = "Save", required = true)]
    path_game: PathBuf,
    /// New game or attempt to load a game
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    load_game: bool,
}

impl Args {
    /// Path to the game location
    pub fn path_game(&self) -> &PathBuf {
        &self.path_game
    }

    /// if a new game should be started
    pub fn load_game(&self) -> bool {
        self.load_game
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn args_error() {
        let args_res = Args::try_parse_from(vec!["bjg"]);
        assert!(args_res.is_err());
    }

    #[test]
    fn load_get() {
        let args_res = Args::try_parse_from(vec!["bjg", "test"]);
        assert!(args_res.is_ok());
        let args = args_res.unwrap();
        assert_eq!(args.load_game(), false);
        let args_res = Args::try_parse_from(vec!["bjg", "test", "-l"]);
        assert!(args_res.is_ok());
        let args = args_res.unwrap();
        assert_eq!(args.load_game(), true);
    }

    #[test]
    fn path_get() {
        let args_res = Args::try_parse_from(vec!["bft", "test"]);
        assert!(args_res.is_ok());
        let args = args_res.unwrap();
        assert_eq!(args.path_game(), Path::new("test"));
    }
}
