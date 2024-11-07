use std::io;

use crate::cards::{Card, Deck};

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
            "Dealers showing: \n\tFacedown Card\n\t{}",
            computer.hand()[1]
        );
        println!("Your cards: \n\t{}\n\t{}", human.hand()[0], human.hand()[1]);
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
                println!("\t{}", self.human.hand.last().unwrap())
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
        println!("The dealers hand:\n\t{}\n\t{}", hand[0], hand[1]);
        println!("\t================");

        let mut total = 0;
        for card in hand {
            total += card.blackjack_value();
        }

        while total <= 16 {
            self.deck.hit(&mut self.computer);
            // FIX
            println!("\t{}", self.computer.hand().last().unwrap());
            total += self.computer.hand().last().unwrap().blackjack_value();
        }
    }

    pub fn winner(&self) -> bool {}
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
