use std::io;

use crate::cards::{Card, Deck};

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
        println!("Starting new game of BlackJack");
        let mut deck = Deck::new();
        println!("Shuffling and Dealing ..... ");
        deck.shuffle();
        let computer = Player::new(deck.deal(2));
        let human = Player::new(deck.deal(2));
        println!(
            "Dealers showing: \n\t{} HIDDEN\n\t{}",
            "\u{1F0A0}",
            print_card!(computer.hand()[1])
        );
        println!(
            "Your cards: \n\t{}\n\t{}",
            print_card!(human.hand()[0]),
            print_card!(human.hand()[1])
        );
        Self {
            deck,
            human,
            computer,
            turn: true,
        }
    }

    pub fn turn(&mut self) {
        let mut stuck = false;
        while !stuck && !self.human.bust() {
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
                // FIX
                println!("\t{}", print_card!(self.human.hand.last().unwrap()))
            }
            if buffer.to_lowercase().contains("stick") {
                stuck = true
            }
        }
        if self.human.bust() {
            println!("You're Bust.")
        }
    }

    pub fn cpu_turn(&mut self) {
        let mut hand = self.computer.hand();
        println!(
            "The dealers hand:\n\t{}\n\t{}",
            print_card!(hand[0]),
            print_card!(hand[1]),
        );
        println!("\t==================");

        let mut total = 0;
        for card in hand {
            total += card.blackjack_value();
        }

        while total <= 16 {
            self.deck.hit(&mut self.computer);
            // FIX
            println!("\t{}", print_card!(self.computer.hand().last().unwrap()));
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

    pub fn score(&self) -> usize {
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

impl Deck {
    pub fn hit(&mut self, player: &mut Player) {
        Deck::shuffle(self);
        player.hit(Deck::deal(self, 1)[0]);
    }
}

impl Card {
    pub fn blackjack_value(&self) -> usize {
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
