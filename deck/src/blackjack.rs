use std::{cmp::max, fmt::Display, io, usize};

use crate::cards::{Card, Deck};
use crate::terminal::Terminal;

macro_rules! print_card {
    ($card:expr) => {
        format!("{} {}", $card.unicode(), $card)
    };
}

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
        let mut hand = self.computer.hand();
        let mut total = 0;
        for card in hand {
            total += card.blackjack_value();
        }

        while total <= 16 {
            self.deck.hit(&mut self.computer);
            println!("{}", self);
            total += self.computer.hand().last().unwrap().blackjack_value();
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

pub struct Player {
    hand: Vec<Card>,
}

impl Player {
    pub fn new(hand: Vec<Card>) -> Self {
        Self { hand }
    }

    pub fn hand(&self) -> &[Card] {
        &self.hand
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

    use super::*;
    use crate::cards::Suits;

    #[test]
    fn player_new() {
        let p = Player::new(vec![]);
        assert_eq!(p.hand.len(), 0);
    }

    #[test]
    fn player_hand() {
        let p = Player::new(vec![Card::new(Suits::Clubs, 1.into())]);
        assert_eq!(p.hand, p.hand());
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
        assert_eq!(p.hand(), vec![Card::new(Suits::Clubs, 1.into())]);
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
}
