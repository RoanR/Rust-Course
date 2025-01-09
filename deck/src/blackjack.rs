use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;
use std::{fmt::Display, io};

use thiserror::Error;

use crate::cards::{Card, Deck};
use crate::terminal::Terminal;

macro_rules! print_card {
    ($card:expr) => {
        format!("{} {}", $card.unicode(), $card)
    };
}

#[derive(Debug, Error)]
pub enum BlackjackError {
    /// A wrapper for an IO error when attempting to save the current game
    #[error("A Wrapped IO Error, from {e}")]
    IOError {
        /// The IO Error being wrapped
        e: std::io::Error,
    },
}

impl Into<BlackjackError> for io::Error {
    fn into(self) -> BlackjackError {
        BlackjackError::IOError { e: self }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Game {
    deck: Deck,
    human: Player,
    computer: Player,
    turn: bool,
}

impl Game {
    pub fn new() -> Self {
        let mut deck = Deck::new();
        deck.shuffle();
        let computer = Player::new(deck.deal(2));
        let human = Player::new(deck.deal(2));
        let game = Self {
            deck,
            human,
            computer,
            turn: true,
        };
        println!("{}", game);
        game
    }

    pub fn turn(&mut self) {
        let mut stuck = false;
        while !stuck && !self.human.bust() {
            println!("{}", self);
            let mut buffer = String::new();
            while !buffer.to_lowercase().contains("twist")
                && !buffer.to_lowercase().contains("stick")
            {
                buffer = String::new();
                println!("Stick or Twist?");
                io::stdin().read_line(&mut buffer);
            }

            if buffer.to_lowercase().contains("twist") {
                self.deck.hit(&mut self.human);
                println!("{}", self);
            }
            if buffer.to_lowercase().contains("stick") {
                stuck = true
            }
        }
        if self.human.bust() {
            println!("{}", self);
        }
        self.turn = false;
    }

    pub fn cpu_turn(&mut self) {
        println!("{}", self);
        while self.computer.score() <= 16 {
            self.deck.hit(&mut self.computer);
            println!("{}", self);
        }
    }

    pub fn human_win(&self) -> bool {
        if self.human.bust()
            || (self.human.score() <= self.computer.score() && !self.computer.bust())
        {
            false
        } else {
            true
        }
    }

    pub fn save(&self, filename: PathBuf) -> Result<(), BlackjackError> {
        let mut f = File::create(filename).map_err(|e| e.into())?;
        let to_write: Vec<u8> = self.clone().into();
        f.write_all(&to_write).map_err(|e| e.into())?;
        Ok(())
    }

    pub fn load(filename: PathBuf) -> Result<Game, BlackjackError> {
        let f = File::open(filename).map_err(|e| e.into())?;
        let mut reader = BufReader::new(f);

        let mut buf = vec![];
        reader.read_to_end(&mut buf).map_err(|e| e.into())?;
        Ok(Game::from(buf))
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut t = Terminal::new();

        // Clear the Terminal
        t.clear();

        // Title
        t.divider(f)?;
        t.centre_text(f, "Blackjack".to_string())?;
        t.divider(f)?;

        // Display Players
        // If first turn - Hide the dealers cards
        let mut text = vec![];
        if self.turn {
            text.push(format!(
                "The Dealer:\n\u{1F0A0} Hidden Card\n{}",
                print_card!(self.computer.hand[1])
            ));
        } else {
            text.push(format!("The Dealer:\n{}", self.computer));
        }
        let bust = if self.human.bust() { "Bust\n" } else { "" };
        text.push(format!("The Player:\n{}{}", self.human, bust));
        t.column_text(f, text)
    }
}

impl Into<Vec<u8>> for Game {
    fn into(self) -> Vec<u8> {
        let mut vgame = vec![];
        vgame.push(if self.turn { 0b1100_0001 } else { 0b1100_0000 });

        for card in &self.computer.hand {
            vgame.push(Into::<u8>::into(*card) + (0b01 << 6));
        }

        for card in &self.human.hand {
            vgame.push(Into::<u8>::into(*card) + (0b10 << 6));
        }

        vgame.append(&mut self.deck.into());
        vgame
    }
}

impl From<Vec<u8>> for Game {
    fn from(value: Vec<u8>) -> Self {
        let turn = if value[0] == 0b1100_0001 { true } else { false };
        let mut pcards = vec![];
        let mut ccards = vec![];
        let mut dcards = vec![];

        for card in value {
            match card >> 6 {
                0b00 => dcards.push(Card::from(card)),
                0b01 => ccards.push(Card::from(card)),
                0b10 => pcards.push(Card::from(card)),
                _ => continue,
            };
        }
        let computer = Player::new(ccards);
        let human = Player::new(pcards);
        let deck = Deck::new_set(dcards);

        Self {
            computer,
            deck,
            human,
            turn,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Player {
    hand: Vec<Card>,
}

impl Player {
    pub fn new(hand: Vec<Card>) -> Self {
        Self { hand }
    }

    pub fn bust(&self) -> bool {
        let mut total = 0;
        for card in &self.hand {
            if card.value() > 10 {
                total += 10;
            } else {
                total += card.value();
            }
        }
        total > 21
    }

    pub fn hit(&mut self, card: Card) {
        self.hand.push(card);
    }

    pub fn score(&self) -> u32 {
        let mut ace_count = 0;
        let mut total = 0;
        for card in &self.hand {
            if card.blackjack_value() == 1 {
                ace_count += 1;
            } else {
                total += card.blackjack_value();
            }
        }
        total += ace_count;
        while ace_count != 0 {
            if total + 10 <= 21 {
                total += 10;
            }
            ace_count -= 1;
        }
        total
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in &self.hand {
            writeln!(f, "{} {}", c.unicode(), c)?;
        }
        Ok(())
    }
}

impl Deck {
    pub fn hit(&mut self, player: &mut Player) {
        Deck::shuffle(self);
        player.hit(Deck::deal(self, 1)[0]);
    }
}

impl Card {
    pub fn blackjack_value(&self) -> u32 {
        if self.value() > 10 {
            10
        } else {
            self.value()
        }
    }
}

#[cfg(test)]
mod tests {

    use tempfile::NamedTempFile;

    use super::*;
    use crate::cards::Suits;

    #[test]
    fn player_new() {
        let p = Player::new(vec![]);
        assert_eq!(p.hand.len(), 0);
    }

    #[test]
    fn player_bust() {
        let mut p = Player::new(vec![
            Card::new(Suits::Clubs, 10.into()),
            Card::new(Suits::Clubs, 11.into()),
        ]);
        assert!(!p.bust());
        p.hand.push(Card::new(Suits::Clubs, 12.into()));
        assert!(p.bust());
    }

    #[test]
    fn player_hit() {
        let mut p = Player::new(vec![]);
        p.hit(Card::new(Suits::Clubs, 1.into()));
        assert_eq!(p.hand, vec![Card::new(Suits::Clubs, 1.into())]);
    }

    #[test]
    fn player_score() {
        let mut p = Player::new(vec![Card::new(Suits::Clubs, 5.into())]);
        assert_eq!(p.score(), 5);
        p.hand.push(Card::new(Suits::Diamonds, 5.into()));
        assert_eq!(p.score(), 10);
        p.hand.push(Card::new(Suits::Diamonds, 0.into()));
        assert_eq!(p.score(), 10);
        p.hand.push(Card::new(Suits::Clubs, 1.into()));
        assert_eq!(p.score(), 21);
        p.hand.push(Card::new(Suits::Diamonds, 1.into()));
        assert_eq!(p.score(), 12);
    }

    #[test]
    fn deck_hit() {
        let mut deck = Deck::new();
        let mut player = Player::new(vec![]);

        assert_eq!(player.hand.len(), 0);
        deck.hit(&mut player);
        assert_eq!(player.hand.len(), 1);
    }

    #[test]
    fn card_blackjack_value() {
        let mut deck = Deck::new();
        for c in deck.deal(52) {
            if c.value() >= 10 {
                assert_eq!(c.blackjack_value(), 10)
            } else {
                assert_eq!(c.value(), c.blackjack_value());
            }
        }
    }

    #[test]
    fn game_new() {
        let game = Game::new();
        assert!(game.turn);
        assert_eq!(game.human.hand.len(), 2);
        assert_eq!(game.computer.hand.len(), 2);
    }

    #[test]
    fn game_vec() {
        let game = Game::new();
        let v: Vec<u8> = game.clone().into();
        let vgame = Game::from(v);
        assert_eq!(game, vgame);
    }

    #[test]
    fn game_save_load() {
        let game = Game::new();
        let f = NamedTempFile::new().unwrap();
        assert!(game.save(f.path().to_path_buf()).is_ok());
        let lgame = Game::load(f.path().to_path_buf()).unwrap();
        assert_eq!(lgame, game);
    }
}
